# convr

## What This Is

convr is a universal local file converter CLI written in Rust. It converts documents, images, and data formats with a single command (`convr --to <format> <inputs>`). It runs entirely offline — no cloud, no runtime dependencies — and produces a single static binary per platform.

## Core Value

A user can point convr at any file and get the format they need, locally, without installing anything else.

## Requirements

### Validated

- ✓ Image conversion: PNG ↔ JPEG ↔ WebP ↔ GIF ↔ BMP ↔ TIFF, SVG → raster, any → AVIF — existing
- ✓ Data format conversion: JSON ↔ YAML ↔ TOML ↔ CSV ↔ XML — existing
- ✓ Document conversion: MD → HTML, HTML → text, MD → text, text → HTML — existing
- ✓ Batch conversion with glob patterns (`*.png`) — existing
- ✓ Parallel batch execution (rayon) — existing
- ✓ Magic-byte format detection — existing
- ✓ Output directory flag (`--out`) — existing

### Active

- [ ] C library backends: libheif (HEIC/HEIF), libvips (raster ops), MuPDF (PDF engine) — Phase 1
- [ ] Document expansion: DOCX read/write, EPUB, HTML→PDF, PDF→Markdown, ODT — Phase 2
- [ ] Image quality & format coverage: HEIC→raster, JPEG XL, PSD, TIFF multipage, GIF→video, resize/quality flags, batch rename — Phase 3
- [ ] UX & distribution: `--list-formats`, `--info`, shell completions, config file, Windows installer, GitHub Actions CI, static release binaries — Phase 4
- [ ] Advanced: pipe support, watch mode, preset profiles, plugin system, GUI wrapper — Phase 5

### Out of Scope

- YAML comments/anchors — lost through serde serialization, no workaround
- SVG → HEIC — blocked on libheif (Phase 1 dependency)
- Cross-category conversion (image → document or document → image) — intentionally unsupported
- Cloud formats (Google Docs, Notion export) — local-only tool by design

## Context

- v0.1.0 is working and structured: `main.rs` (CLI), `batch.rs` (glob + parallel), `convert.rs` (dispatch), `detect.rs` (magic bytes), `formats.rs` (format enum), `converters/{data,document,image}.rs`
- C libraries (libvips, libheif, MuPDF, libwebp, mozjpeg) are listed in Cargo.toml as optional features — not yet wired up
- Build requires cmake, pkg-config, clang/LLVM for C library bindings
- Target: single static binary for Windows, Linux, macOS

## Constraints

- **Tech stack**: Rust — no runtime, statically linked C libraries via FFI
- **Distribution**: Single binary, no install scripts required
- **Platform**: Windows, Linux, macOS parity
- **Scope**: Local-only — no network access, no cloud dependencies

## Key Decisions

| Decision | Rationale | Outcome |
|----------|-----------|---------|
| Pure-Rust image layer first, C libs behind feature flags | Ship working binary without C build complexity; unlock advanced formats incrementally | — Pending |
| rayon for batch parallelism | Simple data-parallel model, fits conversion workload well | — Pending |
| infer crate for magic-byte detection | Avoids relying solely on file extensions | — Pending |
| MuPDF over headless Chrome for PDF | Statically linkable, no browser dependency | — Pending |

---
*Last updated: 2026-03-20 after initialization*
