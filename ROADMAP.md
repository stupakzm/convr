# convr Roadmap

## Current State (v0.1.0)

Working conversions via `convr --to <format> <inputs> [--out <dir>]`:

| Domain | Conversions |
|---|---|
| Images | PNG ↔ JPEG ↔ WebP ↔ GIF ↔ BMP ↔ TIFF, SVG → raster, any → AVIF |
| Data | JSON ↔ YAML ↔ TOML ↔ CSV ↔ XML |
| Documents | MD → HTML, HTML → text, MD → text, text → HTML |

---

## Phase 1 — C Library Backends

Hook up the statically linked C libraries behind feature flags.

- [ ] **libheif** — HEIC/HEIF input (iOS camera format, currently no Rust alternative)
- [ ] **libvips** — replace `image` crate for raster ops (3–4x faster, 300+ formats)
- [ ] **MuPDF** — PDF read/write/render; unlocks: PDF → text, PDF → image, HTML → PDF

Build target: single static binary per platform (Windows, Linux, macOS).

---

## Phase 2 — Document Expansion

Currently limited to Markdown ↔ HTML ↔ text. Expand to office formats.

- [ ] **DOCX read** — extract text + structure from Word documents
- [ ] **DOCX write** — generate Word documents from Markdown/HTML
- [ ] **EPUB read/write** — ebook format (via `epub` crate or Calibre CLI fallback)
- [ ] **HTML → PDF** — requires MuPDF or headless Chrome (via `headless_chrome` crate)
- [ ] **PDF → Markdown** — text extraction with layout hints (requires MuPDF)
- [ ] **ODT support** — OpenDocument text (LibreOffice format)

---

## Phase 3 — Image Quality & Format Coverage

- [ ] **HEIC → raster** — depends on Phase 1 libheif
- [ ] **JPEG XL** — encode/decode via `jxl-oxide` (pure Rust decoder) + libjxl bindings
- [ ] **PSD read** — Photoshop files (via `psd` crate)
- [ ] **TIFF multipage** — export each page as separate image
- [ ] **GIF → MP4/WebM** — animated format conversion (requires FFmpeg or pure Rust encoder)
- [ ] **Resize / quality flags** — `--width 800`, `--quality 85` alongside `--to`
- [ ] **Batch rename** — `--stem "{name}-converted"` pattern for output filenames

---

## Phase 4 — UX & Distribution

- [ ] **`--list-formats`** — print all supported input/output formats
- [ ] **`--info`** — show detected format + metadata of a file without converting
- [ ] **Shell completions** — generate completions for bash/zsh/fish/PowerShell via clap
- [ ] **Config file** — `~/.config/convr/config.toml` for default quality, out dir, etc.
- [ ] **Windows installer** — MSI / winget package
- [ ] **GitHub Actions CI** — build + test on Windows, Linux, macOS
- [ ] **Static release binaries** — upload to GitHub Releases on tag

---

## Phase 5 — Advanced / Stretch

- [ ] **Pipe support** — `cat file.md | convr --from md --to html > out.html`
- [ ] **Watch mode** — `convr --watch --to webp ./images/` (re-convert on file change)
- [ ] **Preset profiles** — `--preset web` (WebP, quality 80), `--preset print` (PDF, 300dpi)
- [ ] **Plugin system** — drop a `.wasm` or `.dll` into a plugins dir to add custom converters
- [ ] **GUI wrapper** — optional Tauri frontend for drag-and-drop batch conversion

---

## Known Gaps / Won't Fix (v1)

- **YAML comments/anchors** — lost through conversion (serde strips them; no workaround)
- **SVG → HEIC** — blocked on libheif Phase 1
- **Cross-category** — image → document or document → image intentionally not supported
- **Cloud formats** — Google Docs, Notion export, etc. — out of scope (local-only tool)
