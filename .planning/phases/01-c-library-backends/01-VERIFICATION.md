---
phase: 01-c-library-backends
status: gaps_found
verified: 2026-03-20
verifier: orchestrator-inline
---

# Phase 01 Verification: C Library Backends

## Goal Assessment

**Phase Goal:** convr can statically link libheif, libvips, and MuPDF so that advanced format conversion is possible without runtime dependencies

**Verdict: GAPS FOUND** — All code is wired correctly and the architecture is sound, but actual compilation with the C library feature flags fails on this Windows MSVC environment due to missing native build tooling. The phase code is complete; the environment is not configured.

---

## Must-Haves Check

### CLIB-01: libheif backend (heif feature)

| Requirement | Status | Evidence |
|------------|--------|----------|
| Cargo.toml has `libheif-rs = { version = "2.7", optional = true, features = ["embedded-libheif"] }` | ✓ | Grep confirms |
| `heif = ["dep:libheif-rs"]` in features | ✓ | Grep confirms |
| `Format::Heif` variant in formats.rs | ✓ | 4 occurrences |
| `from_extension` maps `heic`/`heif` → `Format::Heif` | ✓ | Grep confirms |
| `src/converters/image_heif.rs` with `decode_heif` | ✓ | File exists |
| `image.rs` routes `Format::Heif` through `image_heif::decode_heif` | ✓ | Grep confirms |
| Non-feature build gives clear error for HEIF input | ✓ | `anyhow::bail!("HEIF support requires building with --features heif")` |
| `cargo check --features heif` exits 0 | ✗ | Fails: libheif-sys requires cmake or vcpkg on Windows MSVC |
| A .heic file decodes to DynamicImage via libheif-rs | ✗ | Unverifiable — feature build fails |

**CLIB-01 Status: PARTIAL** — Code complete, build environment insufficient

### CLIB-02: libvips backend (vips feature)

| Requirement | Status | Evidence |
|------------|--------|----------|
| `libvips = { version = "1.7", optional = true }` in Cargo.toml | ✓ | Grep confirms |
| `vips = ["dep:libvips"]` in features | ✓ | Grep confirms |
| `src/backends/mod.rs` has `vips_app()` with OnceLock<VipsApp> | ✓ | Grep confirms |
| `src/converters/image_vips.rs` has `load_via_vips` and `convert_via_vips` | ✓ | File exists |
| `image.rs` routes raster-to-raster through `image_vips::convert_via_vips` | ✓ | Grep confirms |
| `cargo check --features vips` exits 0 | ✗ | Fails: libvips crate has 282 Win32 type errors (i64/i32 mismatches) |
| libvips loads an image and reports dimensions | ✗ | Unverifiable |

**CLIB-02 Status: PARTIAL** — Code complete, libvips crate incompatible with Windows MSVC

### CLIB-03: MuPDF backend (mupdf-backend feature)

| Requirement | Status | Evidence |
|------------|--------|----------|
| `mupdf = { version = "0.6", optional = true }` in Cargo.toml | ✓ | Grep confirms |
| `mupdf-backend = ["dep:mupdf"]` in features | ✓ | Grep confirms |
| `src/converters/document_pdf.rs` has `open_pdf`, `pdf_page_count`, `render_page_to_png` | ✓ | File exists |
| `document.rs` routes PDF→PNG/JPEG through `document_pdf::render_page_to_png` | ✓ | Grep confirms |
| `cargo check --features mupdf-backend` exits 0 | ✗ | Fails: bindgen requires MSVC-compatible libclang.dll; MinGW version not ABI-compatible |
| MuPDF opens a PDF and reports page count | ✗ | Unverifiable |

**CLIB-03 Status: PARTIAL** — Code complete, missing MSVC-compatible LLVM libclang

---

## Gaps Found

### Gap 1: Windows MSVC build tooling for `--features heif`
- **What's missing:** cmake (from cmake.org) or vcpkg with `libheif:x64-windows`
- **Fix:** Install cmake from cmake.org and ensure it's on PATH; or install vcpkg and run `vcpkg install libheif:x64-windows`
- **Scope:** Environment fix only, no code changes needed

### Gap 2: libvips crate Windows MSVC compatibility
- **What's missing:** The `libvips = "1.7"` crate has 282 Win32 type incompatibilities (generated bindings use i64 where Win32 expects i32)
- **Fix:** Either (a) downgrade to `libvips = "1.6"` if that has Windows fixes, (b) switch Rust toolchain to GNU/MinGW, or (c) wait for upstream crate fix
- **Scope:** Possible Cargo.toml version change and/or toolchain switch

### Gap 3: MuPDF bindgen requires MSVC-compatible LLVM
- **What's missing:** `libclang.dll` from LLVM for Windows (MSVC ABI), not the MinGW version
- **Fix:** Install LLVM for Windows from llvm.org (select "Windows installer"), set `LIBCLANG_PATH=C:\Program Files\LLVM\bin`
- **Scope:** Environment install + env var, no code changes needed

---

## Human Verification Items

1. **Install LLVM for Windows** (llvm.org → Windows installer) and set `LIBCLANG_PATH`, then re-run `cargo check --features mupdf-backend`
2. **Install cmake** (cmake.org) and re-run `cargo check --features heif`
3. After tooling is installed, run `cargo test --features heif,mupdf-backend --test heif_test --test mupdf_test` to verify decode and render work end-to-end
4. For libvips: investigate whether `libvips = "1.6"` or a GNU toolchain resolves the Win32 type errors

---

## Automated Checks: PASSED

| Check | Result |
|-------|--------|
| All 13 required artifacts exist | ✓ |
| `cargo check` (no features) | ✓ |
| `cargo test --test heif_test` (no features) | ✓ |
| `cargo test --test vips_test` (no features) | ✓ |
| `cargo test --test mupdf_test` (no features) | ✓ |
| `cargo test --test all_backends_test` (no features) | ✓ |
| HEIF error message when feature disabled | ✓ |
| PDF error message when mupdf-backend disabled | ✓ |
