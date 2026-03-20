# Technology Stack

**Analysis Date:** 2026-03-20

## Languages

**Primary:**
- Rust (edition 2021) - all application code in `src/`

**Secondary:**
- C (optional, via FFI feature flags) - statically linked backends: libvips, libheif, MuPDF

## Runtime

**Environment:**
- Native binary — no runtime VM or interpreter

**Package Manager:**
- Cargo (Rust toolchain)
- Lockfile: `Cargo.lock` present and committed

## Frameworks

**Core:**
- None - pure Rust binary, no application framework

**CLI:**
- `clap` v4 (derive feature) - argument parsing via `#[derive(Parser)]` on `struct Cli` in `src/main.rs`

**Testing:**
- Not yet configured — no test files or test runner config present

**Build/Dev:**
- `cargo build --release` — produces `target/release/convr`
- Optional C build tools for feature-flagged backends: `cmake`, `pkg-config`, `clang`/LLVM

## Key Dependencies

**CLI & UX:**
- `clap` v4 (derive) - argument parsing; `src/main.rs`
- `indicatif` v0.17 - progress bars during batch conversion; `src/batch.rs`
- `anyhow` v1 - ergonomic error propagation with `?` throughout all modules
- `glob` v0.3 - shell-style glob expansion for input file patterns; `src/batch.rs`

**Parallelism:**
- `rayon` v1 - parallel batch file processing via `par_iter()`; `src/batch.rs`

**Format Detection:**
- `infer` v0.16 - magic-byte MIME detection as fallback when file extension is absent/wrong; `src/detect.rs`

**Image Processing:**
- `image` v0.25 (png, jpeg, gif, webp, tiff, bmp, ico, rayon features, default-features off) - primary raster image codec; `src/converters/image.rs`
- `resvg` v0.44 - SVG parsing and rasterization via usvg; `src/converters/image.rs`
- `tiny-skia` v0.11 - pixel buffer (`Pixmap`) used by resvg rendering pipeline; `src/converters/image.rs`
- `oxipng` v9 (parallel, zopfli, filetime features) - post-encode PNG optimization at preset level 3; `src/converters/image.rs`
- `ravif` v0.11 (threading feature) - AVIF encoding with rav1e backend at quality 80, speed 4; `src/converters/image.rs`
- `rgb` v0.8 - pixel type (`RGBA8`) used as interface to ravif; `src/converters/image.rs`

**Data Formats:**
- `serde` v1 (derive) - serialization trait derivation; `src/converters/data.rs`
- `serde_json` v1 - JSON parse/emit; used as universal intermediate value (`serde_json::Value`) for all data conversions; `src/converters/data.rs`
- `serde_yaml` v0.9 - YAML parse/emit; `src/converters/data.rs`
- `toml` v0.8 (parse, display features) - TOML parse/emit; `src/converters/data.rs`
- `csv` v1 - CSV reader/writer; `src/converters/data.rs`
- `quick-xml` v0.36 (serialize feature) - event-driven XML parse/emit; `src/converters/data.rs`

**Document Formats:**
- `pulldown-cmark` v0.12 - CommonMark Markdown parser and HTML emitter; `src/converters/document.rs`

## Feature Flags (Optional C Backend Extensions)

Defined in `Cargo.toml` under `[features]`, none enabled by default:

| Flag | C Library | Unlocks |
|------|-----------|---------|
| `vips` | libvips | 300+ image formats, 3-4x faster raster ops |
| `mupdf` | MuPDF | PDF read/write/render |
| `heif` | libheif | HEIC/HEIF input (iOS camera format) |

Enable with: `cargo build --release --features vips,mupdf,heif`

## Configuration

**Environment:**
- No environment variables — tool is entirely CLI-flag driven
- No `.env` file present

**Build:**
- `Cargo.toml` — single manifest at project root
- `Cargo.lock` — dependency lockfile at project root
- No `rust-toolchain.toml` or `.nvmrc` present — uses ambient Rust toolchain

## Platform Requirements

**Development:**
- Rust toolchain (stable, edition 2021)
- For C feature flags: `cmake`, `pkg-config`, `clang`/LLVM

**Production:**
- Single native binary: `target/release/convr`
- Deployment target: Windows, Linux, macOS (cross-platform; static binary planned per ROADMAP.md)
- No runtime dependencies when built without C feature flags (pure Rust)

---

*Stack analysis: 2026-03-20*
