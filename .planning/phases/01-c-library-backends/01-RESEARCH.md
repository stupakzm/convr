# Phase 1: C Library Backends - Research

**Researched:** 2026-03-20
**Domain:** Rust FFI / static linking — libheif, libvips, MuPDF
**Confidence:** MEDIUM-HIGH (core crate facts HIGH; cross-platform build matrix MEDIUM; link-conflict risk MEDIUM)

---

<phase_requirements>
## Phase Requirements

| ID | Description | Research Support |
|----|-------------|-----------------|
| CLIB-01 | convr links libheif statically and can read HEIC/HEIF input files | `libheif-rs` 2.7.0 + `embedded-libheif` feature compiles libheif from bundled source; codec deps (libde265, libaom) still need system packages or separate bundling |
| CLIB-02 | convr links libvips statically and uses it for raster image operations (replacing the image crate) | `libvips` crate (olxgroup bindings, v1.7.1) wraps libvips via pkg-config; true static link requires pre-built static libs or build-from-source; LGPL-2.1 requires source offer |
| CLIB-03 | convr links MuPDF statically and supports PDF read, write, and render | `mupdf` 0.6.0 bundles all MuPDF C source and compiles it via Make/MSBuild; fully self-contained static link; AGPL-3.0 license applies to convr distribution |
</phase_requirements>

---

## Summary

Phase 1 must wire three large C libraries into a Rust binary behind optional Cargo feature flags (`heif`, `vips`, `mupdf`). Each library has an established Rust binding, but the path to a *truly* static, zero-system-dependency binary is different for each one.

**libheif** (`libheif-rs` 2.7.0) provides an `embedded-libheif` feature that compiles the library itself from bundled source, but the codec backends — libde265 (HEIC decode) and libaom (AVIF decode) — remain dynamically linked. Getting a fully static codec chain on all three platforms requires either vendoring those decoders too or accepting that the CI runners must have codec packages installed. libheif is LGPL-2.1; static linking creates a source-offer obligation.

**libvips** (`libvips` crate, olxgroup bindings v1.7.1) is the most complex to link statically. The library is built on GLib, which itself has deep platform dependencies. The Rust crate uses pkg-config and assumes libvips is already installed on the system. True static linkage requires pre-building libvips and all its dependencies (glib, jpeg, webp, etc.) as `.a` archives, which is substantial CI work. libvips is LGPL-2.1.

**MuPDF** (`mupdf` 0.6.0, `mupdf-sys` 0.6.0) bundles the entire MuPDF C source tree inside the Rust crate and compiles it via `make` (Linux/macOS) or MSBuild (Windows). This is the most self-contained of the three — no pre-installed C library needed. The cost is that MuPDF is AGPL-3.0; any convr binary distributed publicly must itself be AGPL-3.0 or a commercial Artifex license must be obtained.

**Primary recommendation:** Wire all three behind feature flags now. Use `embedded-libheif` for CLIB-01 (accept dynamic codec deps on CI). Require system libvips for CLIB-02 initially (document `brew install vips` / `apt install libvips-dev`). Use `mupdf` crate's bundled build for CLIB-03. Treat fully static bundling of codec/vips deps as a Phase 4 distribution concern.

---

## Standard Stack

### Core

| Library | Version | Purpose | Why Standard |
|---------|---------|---------|--------------|
| `libheif-rs` | 2.7.0 | Safe Rust wrapper for libheif HEIC/HEIF decode/encode | Actively maintained (2026-02-25 release), wraps libheif 1.21.2, provides `embedded-libheif` feature |
| `libheif-sys` | 5.2.0+1.21.2 | FFI bindings (pulled as transitive dep of `libheif-rs`) | Same author; handles pkg-config (Linux) and vcpkg (Windows) discovery |
| `libvips` | 1.7.1 (olxgroup) | Safe bindings for libvips raster operations | Generated from libvips 8.17.3 introspection API; most complete coverage |
| `mupdf` | 0.6.0 | Safe Rust wrapper for MuPDF PDF operations | Bundles full MuPDF source; compiles without system install |
| `mupdf-sys` | 0.6.0 | FFI + bundled MuPDF C source (pulled transitively) | Uses `cc` + `bindgen`; `make` on Linux/macOS, MSBuild on Windows |

### Supporting Build Tools

| Library | Version | Purpose | When to Use |
|---------|---------|---------|-------------|
| `cmake` crate | 0.1.x | Drive CMake-based sub-builds from build.rs | If vendoring libde265/libaom for fully static HEIC |
| `cc` crate | 1.x | Compile C/C++ source from build.rs | Used internally by mupdf-sys; needed if writing custom wrappers |
| `pkg-config` crate | 0.3.x | Locate system libraries at build time | Used by libheif-sys (Linux) and libvips |
| `bindgen` crate | 0.72.x | Generate FFI bindings from C headers | Build dependency of both libheif-sys and mupdf-sys |

### Alternatives Considered

| Instead of | Could Use | Tradeoff |
|------------|-----------|----------|
| `libheif-rs` | Raw `libheif-sys` | More control over FFI; no safe wrapper overhead; not worth it |
| `libvips` (olxgroup) | `houseme/vips-rs` | Newer fork, but smaller community; olxgroup has more production use |
| `mupdf` (messense) | `pdfium-render` (Chromium PDFium) | PDFium is BSD-licensed (no AGPL); harder to bundle statically; less Rust coverage |
| `mupdf` (messense) | `lopdf` (pure Rust) | No AGPL; much lower fidelity — no rendering, limited write support |

**Installation (Cargo.toml additions):**
```toml
[dependencies]
libheif-rs = { version = "2.7", optional = true }
libvips     = { version = "1.7", optional = true }  # olxgroup crate name is "libvips"
mupdf       = { version = "0.6", optional = true }

[features]
heif  = ["dep:libheif-rs"]
vips  = ["dep:libvips"]
mupdf = ["dep:mupdf"]
```

**Version verification:** Confirmed against crates.io and docs.rs on 2026-03-20.

---

## Architecture Patterns

### Recommended Project Structure
```
src/
├── converters/
│   ├── image.rs          # existing — add heif branch behind #[cfg(feature = "heif")]
│   ├── image_heif.rs     # NEW: HEIC/HEIF read via libheif-rs
│   ├── image_vips.rs     # NEW: vips raster routing when feature = "vips"
│   ├── document.rs       # existing — add pdf branch behind #[cfg(feature = "mupdf")]
│   └── document_pdf.rs   # NEW: PDF open/render/write via mupdf crate
├── backends/
│   └── mod.rs            # NEW: re-export feature-gated back-end availability flags
└── formats.rs            # add Heif variant to Format enum
```

### Pattern 1: Feature-Gated Module Routing

Use `#[cfg(feature = "heif")]` on entire modules to keep dead-code warnings away and ensure non-feature builds still compile cleanly.

**What:** Route conversion calls through a feature-gated function that returns `Err` with a clear message when the feature is absent.
**When to use:** Every entry point that touches a C library.

```rust
// src/converters/image.rs
pub fn convert(input: &Path, src: &Format, output: &Path, target: &Format) -> Result<()> {
    match src {
        Format::Heif => {
            #[cfg(feature = "heif")]
            return image_heif::decode(input, output, target);
            #[cfg(not(feature = "heif"))]
            anyhow::bail!("HEIF support requires the `heif` feature flag");
        }
        // ... existing arms
    }
}
```

### Pattern 2: sys-Crate links Key

The `links = "heif"` key in libheif-sys prevents Cargo from linking libheif twice if two crates both depend on it. This is automatically handled by the `-sys` crates; convr does not need to set it.

### Pattern 3: libheif Static Embedding

Enable `embedded-libheif` in libheif-sys to compile libheif from source. Accept that codec backends (libde265, libaom) remain dynamic — CI runners provide them.

```toml
# Cargo.toml
[dependencies]
libheif-rs = { version = "2.7", optional = true, features = ["embedded-libheif"] }
```

**Warning:** `embedded-libheif` only affects `libheif` itself, not decoders. On a clean Linux CI runner, install `libde265-dev libaom-dev` before build.

### Pattern 4: MuPDF Bundled Build

`mupdf-sys` includes the full MuPDF source tree (C code in `mupdf/` directory) and compiles it during `cargo build`. No system install needed. Build time is ~3-5 minutes on first compile; incremental rebuilds are fast.

```rust
// src/converters/document_pdf.rs  (feature = "mupdf")
use mupdf::{Document, Matrix, Pixmap, ColorSpace};

pub fn open_pdf(path: &Path) -> anyhow::Result<Document> {
    Ok(Document::open(path.to_str().unwrap())?)
}

pub fn render_page(doc: &Document, page_idx: i32, dpi: f32) -> anyhow::Result<Pixmap> {
    let page = doc.load_page(page_idx)?;
    let scale = dpi / 72.0;
    let matrix = Matrix::new_scale(scale, scale);
    let bounds = page.bounds()?;
    let pixmap = Pixmap::new_with_rect(&ColorSpace::device_rgb(), bounds.transform(&matrix), false)?;
    // ... render
    Ok(pixmap)
}
```

### Pattern 5: libvips Initialization

libvips requires an explicit startup/shutdown call. The `libvips` crate handles this via a `VipsApp` guard; hold it for the program's lifetime.

```rust
// src/backends/mod.rs  (feature = "vips")
use libvips::VipsApp;
static VIPS: std::sync::OnceLock<VipsApp> = std::sync::OnceLock::new();

pub fn vips() -> &'static VipsApp {
    VIPS.get_or_init(|| VipsApp::new("convr", false).expect("libvips init failed"))
}
```

### Anti-Patterns to Avoid

- **Calling libvips functions before `VipsApp::new`:** libvips is not thread-safe before initialization; calling any vips API before init causes undefined behavior.
- **Mixing static and dynamic libheif:** Do not set `embedded-libheif` and also have libheif installed via pkg-config; the linker may duplicate symbols. Pick one source.
- **Enabling all three features simultaneously without testing link order:** zlib and libjpeg appear in both libvips and mupdf-sys. The `-sys` crates expose them as `sys-lib-*` features; if two sys crates both try to link the same symbol from separate `.a` files, duplicate symbol errors occur. Use `sys-lib-zlib = false` / `sys-lib-libjpeg = false` in mupdf-sys features when system libs are present.
- **Using `cargo test` without `--features`:** Tests gated on C library features will silently be skipped unless the feature is explicitly passed: `cargo test --features heif,vips,mupdf`.

---

## Don't Hand-Roll

| Problem | Don't Build | Use Instead | Why |
|---------|-------------|-------------|-----|
| HEIC/HEIF decoding | Custom HEIF parser | `libheif-rs` with `embedded-libheif` | HEIF container is ISO 14496-12; codec layer (HEVC/AV1) is 50k+ LOC |
| PDF parsing/rendering | Custom PDF parser | `mupdf` crate | PDF has 750+ page spec; MuPDF took 20 years to get right |
| Large image processing pipeline | Custom vips-like ops | `libvips` crate | Memory-mapped streaming pipeline; reimplementing causes OOM on large images |
| Build-time header discovery | Hand-written pkg-config shell | `pkg-config` crate | Handles cross-compilation sysroot, multiarch dirs, and MSVC correctly |
| C header bindings | Hand-written `extern "C"` declarations | `bindgen` (via `-sys` crates) | ABI stability — manual declarations rot silently when C headers change |
| Duplicate library prevention | Custom link dedup | Cargo `links` key convention | Cargo enforces one `links = "X"` per build; -sys crates use this by design |

**Key insight:** Each of these C libraries represents 5-20 years of edge-case handling. The Rust binding crates exist precisely because hand-rolling FFI against these libraries is a known multi-month project.

---

## Common Pitfalls

### Pitfall 1: libheif Codec Dependencies Not Installed on CI

**What goes wrong:** Build succeeds with `embedded-libheif`, but at runtime `libheif-rs` returns "no suitable decoder found" for `.heic` files because libde265 is absent.
**Why it happens:** `embedded-libheif` only bundles the container layer, not codec backends. Codecs are loaded as plugins or linked separately.
**How to avoid:** Add `apt-get install -y libde265-dev libaom-dev` (or `brew install libde265 aom`) to CI setup steps before `cargo build --features heif`.
**Warning signs:** `HeifError { code: UnsupportedFeature, message: "Unsupported codec: HEVC" }` at test time.

### Pitfall 2: MuPDF AGPL License Propagation

**What goes wrong:** Linking MuPDF statically makes the entire `convr` binary a derivative work under AGPL-3.0. Distributing the binary without providing convr's full source under AGPL is a license violation.
**Why it happens:** AGPL-3.0 requires network-service providers *and* binary distributors to offer source.
**How to avoid:** Either (a) distribute convr source under AGPL-3.0, (b) obtain a commercial Artifex license, or (c) use `lopdf` (pure Rust, MIT) for basic PDF operations and document the limitation.
**Warning signs:** Any GitHub Release binary that ships the `mupdf` feature without source.

### Pitfall 3: libvips LGPL Static Linking Obligation

**What goes wrong:** Static linking libvips (LGPL-2.1) into convr creates an obligation to allow users to relink the binary against a modified libvips. This is difficult with a pre-built binary distribution.
**Why it happens:** LGPL-2.1 §6 requires that the user be able to swap the covered library.
**How to avoid:** For Phase 1, use dynamic linking for libvips (system-installed). If a fully static distribution is required later, consider linking via a shared library wrapper or switching to dynamic loading at runtime.
**Warning signs:** Shipping a static binary with `--features vips` in a GitHub Release.

### Pitfall 4: Duplicate Symbols When All Three Features Are Enabled

**What goes wrong:** `cargo build --features heif,vips,mupdf` fails with linker errors like `duplicate symbol _jpeg_finish_compress`.
**Why it happens:** libvips, libheif, and mupdf-sys all depend on libjpeg and zlib. If mupdf-sys bundles its own copies as static archives and the system also provides them for libvips, the linker sees the same symbol twice.
**How to avoid:** When using all features together, enable `sys-lib-libjpeg` and `sys-lib-zlib` features in mupdf-sys to tell it to use system versions rather than its bundled copies. Verify with `cargo build --features heif,vips,mupdf 2>&1 | grep -i "duplicate"`.
**Warning signs:** Linker error `duplicate symbol` or `multiple definition` mentioning jpeg, z, or freetype.

### Pitfall 5: bindgen Requires Clang/LLVM at Build Time

**What goes wrong:** `cargo build --features heif` fails with `error: failed to run custom build command` because `bindgen` cannot find `libclang`.
**Why it happens:** bindgen uses libclang to parse C headers; if clang is not installed on the build machine, binding generation fails.
**How to avoid:** Add `clang` (or `llvm`) to CI build prerequisites. On macOS, Xcode command-line tools provide it. On Linux, install `clang libclang-dev`.
**Warning signs:** `error: libclang not found` in cargo output.

### Pitfall 6: MuPDF Build Time is Very Long on First Run

**What goes wrong:** CI times out or developer is confused by a 5-10 minute `cargo build`.
**Why it happens:** mupdf-sys compiles the full MuPDF C source tree (hundreds of files) on first build.
**How to avoid:** Cache `~/.cargo/registry` and `target/` in CI. Use `sccache` for distributed caching. Document the expected build time.
**Warning signs:** CI job taking >10 minutes with no progress output.

---

## Code Examples

Verified patterns from official sources and docs.rs:

### HEIF Decode with libheif-rs
```rust
// Source: https://docs.rs/libheif-rs/latest/libheif_rs/
use libheif_rs::{HeifContext, RgbChroma, ColorSpace};

pub fn decode_heif(path: &Path) -> anyhow::Result<image::DynamicImage> {
    let ctx = HeifContext::read_from_file(path.to_str().unwrap())?;
    let handle = ctx.primary_image_handle()?;
    let img = handle.decode(ColorSpace::Rgb(RgbChroma::Rgb), None)?;
    let planes = img.planes();
    let interleaved = planes.interleaved
        .ok_or_else(|| anyhow::anyhow!("expected interleaved plane"))?;
    let width = img.width();
    let height = img.height();
    let data = interleaved.data.to_vec();
    Ok(image::DynamicImage::ImageRgb8(
        image::RgbImage::from_raw(width, height, data)
            .ok_or_else(|| anyhow::anyhow!("buffer size mismatch"))?,
    ))
}
```

### MuPDF: Open, Render a Page, Write PDF
```rust
// Source: https://docs.rs/mupdf/latest/mupdf/
use mupdf::{Document, Matrix, Pixmap, ColorSpace};

pub fn render_pdf_page(pdf_path: &Path, page_idx: i32, dpi: f32)
    -> anyhow::Result<Vec<u8>>
{
    let doc = Document::open(pdf_path.to_str().unwrap())?;
    let page = doc.load_page(page_idx)?;
    let scale = dpi / 72.0;
    let matrix = Matrix::new_scale(scale, scale);
    let bounds = page.bounds()?.transform(&matrix);
    let mut pixmap = Pixmap::new_with_rect(
        &ColorSpace::device_rgb(), bounds, false,
    )?;
    pixmap.clear()?;
    page.run(&mut pixmap.as_device()?, &matrix, None)?;
    Ok(pixmap.samples().to_vec())
}
```

### libvips: Thumbnail via Rust Bindings
```rust
// Source: https://docs.rs/libvips/latest/libvips/
use libvips::{VipsApp, ops};

pub fn vips_resize(input: &Path, output: &Path, width: u32) -> anyhow::Result<()> {
    let _app = VipsApp::new("convr", false)?;
    let img = libvips::VipsImage::new_from_file(input.to_str().unwrap())?;
    let out = ops::thumbnail_image(&img, width as i32)?;
    out.image_write_to_file(output.to_str().unwrap())?;
    Ok(())
}
```

### Cargo Feature Guard Pattern
```rust
// Suppress unused-import warnings when feature is off
#[cfg(feature = "heif")]
mod image_heif;

#[cfg(feature = "mupdf")]
mod document_pdf;
```

---

## State of the Art

| Old Approach | Current Approach | When Changed | Impact |
|--------------|------------------|--------------|--------|
| Dynamic-only libheif linking | `embedded-libheif` feature bundles source | libheif-sys ~4.x (2023) | Eliminates system libheif install requirement (codec deps remain) |
| libheif supports only HEIC | libheif 1.17+ also supports AVIF decode/encode | libheif 1.17 (2023) | Single crate handles both HEIC and AVIF via same API |
| mupdf-rs hand-wrote bindings | bindgen-generated bindings in mupdf-sys | 0.5.x (2024) | Bindings auto-update with MuPDF source upgrades |
| libvips 8.x bindings (augustocdias) | Updated libvips 8.17.3 bindings (olxgroup fork) | 2024-2025 | Covers newer vips operations; old crate was abandoned |
| Cargo 2018 feature syntax | `dep:` optional dependency prefix | Rust 1.60 / Cargo 2022 | `features = ["dep:libheif-rs"]` avoids implicit feature-as-dep |

**Deprecated/outdated:**
- `augustocdias/libvips-rust-bindings` (crates.io: `libvips`): original author abandoned; use `olxgroup-oss/libvips-rust-bindings` fork instead. Both publish to crates.io as `libvips`; check ownership before pinning.
- `libheif-rs` versions <2.0: pre-bindgen, hand-written FFI; do not use.

---

## Open Questions

1. **Full static codec chain for libheif (CLIB-01)**
   - What we know: `embedded-libheif` links libheif itself statically; libde265 and libaom stay dynamic.
   - What's unclear: Whether vendoring libde265 and libaom as git submodules and compiling them via `cmake` crate in a custom build script is the right approach for distribution, or if accepting dynamic codec deps is acceptable.
   - Recommendation: Accept dynamic codec deps for Phase 1 (CI installs packages). Defer full static codec bundling to Phase 4 release engineering.

2. **License strategy for MuPDF (CLIB-03)**
   - What we know: mupdf crate is AGPL-3.0. Linking it into convr makes convr AGPL-3.0.
   - What's unclear: Project's intended license. If convr is meant to be MIT or proprietary, MuPDF is incompatible without a commercial Artifex license.
   - Recommendation: Decide on convr's license before implementing CLIB-03. If open-source AGPL is acceptable, proceed. If MIT distribution is needed, evaluate `lopdf` (pure Rust, MIT) as a limited alternative or plan for Artifex commercial license.

3. **libvips static link strategy for distribution (CLIB-02)**
   - What we know: libvips is LGPL-2.1; static linking creates source-offer obligation. The Rust crate requires system libvips via pkg-config.
   - What's unclear: Whether Phase 1 needs static link or system-dynamic is acceptable.
   - Recommendation: Phase 1 — dynamic link, document `brew install vips` / `apt install libvips-dev`. Phase 4 — evaluate static build using libvips pre-built release archives.

4. **Duplicate symbol conflicts when all three features enabled**
   - What we know: libjpeg and zlib are common deps across all three C libraries. mupdf-sys has `sys-lib-*` feature flags to use system libs.
   - What's unclear: Which combination of feature flags avoids conflicts in practice (needs empirical testing).
   - Recommendation: Wave 0 task — create a test build with `--features heif,vips,mupdf` and resolve any linker conflicts early.

---

## Validation Architecture

### Test Framework

| Property | Value |
|----------|-------|
| Framework | Rust built-in (`cargo test`) |
| Config file | none — standard Cargo test runner |
| Quick run command | `cargo test --features heif,vips,mupdf 2>&1` |
| Full suite command | `cargo test --features heif,vips,mupdf -- --include-ignored` |

### Phase Requirements → Test Map

| Req ID | Behavior | Test Type | Automated Command | File Exists? |
|--------|----------|-----------|-------------------|-------------|
| CLIB-01 | HEIC file decodes to JPEG without error | integration | `cargo test --features heif heif_decode` | ❌ Wave 0 |
| CLIB-01 | Build with `heif` feature compiles cleanly | smoke | `cargo build --features heif` | ❌ Wave 0 |
| CLIB-02 | libvips raster resize produces correct dimensions | integration | `cargo test --features vips vips_resize` | ❌ Wave 0 |
| CLIB-02 | Build with `vips` feature compiles cleanly | smoke | `cargo build --features vips` | ❌ Wave 0 |
| CLIB-03 | PDF opens, page renders to pixmap, writes PDF | integration | `cargo test --features mupdf pdf_roundtrip` | ❌ Wave 0 |
| CLIB-03 | Build with `mupdf` feature compiles cleanly | smoke | `cargo build --features mupdf` | ❌ Wave 0 |
| All | All three features enabled simultaneously link | smoke | `cargo build --features heif,vips,mupdf` | ❌ Wave 0 |

### Sampling Rate
- **Per task commit:** `cargo check --features heif,vips,mupdf`
- **Per wave merge:** `cargo test --features heif,vips,mupdf`
- **Phase gate:** Full suite green before `/gsd:verify-work`

### Wave 0 Gaps

- [ ] `tests/c_backends.rs` — integration tests for CLIB-01, CLIB-02, CLIB-03
- [ ] `tests/fixtures/sample.heic` — minimal HEIC test fixture
- [ ] `tests/fixtures/sample.pdf` — minimal PDF test fixture
- [ ] `tests/fixtures/sample.jpg` — raster fixture for vips resize test
- [ ] CI tooling: add `libde265-dev libaom-dev libvips-dev clang` to GitHub Actions setup step

---

## Sources

### Primary (HIGH confidence)
- [docs.rs/libheif-rs/latest](https://docs.rs/crate/libheif-rs/latest) — version 2.7.0, features including `embedded-libheif`, image integration
- [docs.rs/libheif-sys/latest](https://docs.rs/libheif-sys/latest) — build deps: system-deps, vcpkg, cmake, bindgen; platform behavior
- [github.com/Cykooz/libheif-rs](https://github.com/Cykooz/libheif-rs) — `embedded-libheif` limitation: codec backends not statically bundled
- [github.com/messense/mupdf-rs Cargo.toml](https://github.com/messense/mupdf-rs/blob/main/Cargo.toml) — v0.6.0, AGPL-3.0, default features, serde support
- [github.com/messense/mupdf-rs mupdf-sys/Cargo.toml](https://github.com/messense/mupdf-rs/blob/main/mupdf-sys/Cargo.toml) — bundled MuPDF source, sys-lib-* feature flags, bindgen + cc build deps
- [github.com/messense/mupdf-rs mupdf-sys/build.rs](https://github.com/messense/mupdf-rs/blob/main/mupdf-sys/build.rs) — Make/MSBuild selection, FZ_ENABLE_* defines
- [docs.rs/mupdf/latest](https://docs.rs/mupdf/latest/mupdf/) — Document, Page, Pixmap, DocumentWriter types
- [doc.rust-lang.org/cargo/reference/build-scripts.html](https://doc.rust-lang.org/cargo/reference/build-scripts.html) — `links` key, `rustc-link-lib`, duplicate prevention
- [github.com/olxgroup-oss/libvips-rust-bindings](https://github.com/olxgroup-oss/libvips-rust-bindings) — v1.7.1, libvips 8.17.3, build via Docker

### Secondary (MEDIUM confidence)
- [artifex.com/licensing](https://artifex.com/licensing) — MuPDF dual license: AGPL-3.0 or commercial; verified against official Artifex site
- [libvips LGPL-2.1 static link issue](https://github.com/libvips/build-win64/issues/21) — glib/libcroco must stay as shared libs per LGPL requirement; confirmed against libvips GitHub
- [strukturag/libheif README](https://github.com/strukturag/libheif/blob/master/README.md) — codec plugin system since v1.14.0; WITH_{codec}_PLUGIN CMake option

### Tertiary (LOW confidence)
- [Rust forum: linking libvips statically](https://users.rust-lang.org/t/linking-libvips-statically/19520) — community thread from 2018; static libvips requires all deps as .a; process largely unchanged but versions differ

---

## Metadata

**Confidence breakdown:**
- Standard stack (crate names, versions): HIGH — verified against docs.rs and GitHub on 2026-03-20
- Architecture patterns (feature-gate wiring, init pattern): HIGH — derived from official crate docs and Cargo reference
- MuPDF bundled build behavior: HIGH — confirmed from mupdf-sys Cargo.toml and build.rs source
- libheif codec limitation: HIGH — explicitly documented in libheif-sys README
- libvips static link complexity: MEDIUM — general LGPL + GLib dependency tree is well-established; exact `.a` build process for Phase 4 needs empirical testing
- Link conflict risk (all features enabled): MEDIUM — documented pattern in Rust community; exact mupdf-sys flag combination needs empirical verification
- License implications (AGPL, LGPL): MEDIUM — confirmed from official license pages; specific legal interpretation needs counsel

**Research date:** 2026-03-20
**Valid until:** 2026-06-20 (stable ecosystem; mupdf-rs and libheif-rs release on 1-3 month cadence)
