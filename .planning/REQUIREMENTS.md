# Requirements: convr

**Defined:** 2026-03-20
**Core Value:** A user can point convr at any file and get the format they need, locally, without installing anything else.

## v1 Requirements

### C Library Backends

- [ ] **CLIB-01**: convr links libheif statically and can read HEIC/HEIF input files
- [ ] **CLIB-02**: convr links libvips statically and uses it for raster image operations (replacing the image crate)
- [ ] **CLIB-03**: convr links MuPDF statically and supports PDF read, write, and render

### Documents

- [ ] **DOCS-01**: User can convert DOCX files to text/HTML (read/extract)
- [ ] **DOCS-02**: User can generate DOCX files from Markdown or HTML (write)
- [ ] **DOCS-03**: User can convert EPUB files to text (read)
- [ ] **DOCS-04**: User can generate EPUB files from Markdown or HTML (write)
- [ ] **DOCS-05**: User can convert HTML to PDF (`convr --to pdf input.html`)
- [ ] **DOCS-06**: User can convert PDF to Markdown with layout hints (`convr --to md input.pdf`)

### Images

- [ ] **IMG-01**: User can convert HEIC/HEIF files to any raster format (`convr --to jpeg input.heic`)
- [ ] **IMG-02**: User can encode and decode JPEG XL files
- [ ] **IMG-03**: User can read PSD (Photoshop) files as input for conversion
- [ ] **IMG-04**: User can specify resize dimensions (`--width 800`, `--height 600`)
- [ ] **IMG-05**: User can specify output quality (`--quality 85`)

### UX & Distribution

- [ ] **UX-01**: User can run `convr --list-formats` to see all supported input/output formats
- [ ] **UX-02**: User can run `convr --info <file>` to see detected format and metadata without converting
- [ ] **UX-03**: Shell completions are generated for bash, zsh, fish, and PowerShell
- [ ] **UX-04**: User can configure defaults in `~/.config/convr/config.toml` (output quality, out dir, etc.)
- [ ] **UX-05**: GitHub Actions CI builds and tests convr on Windows, Linux, and macOS on every push
- [ ] **UX-06**: Tagged releases automatically upload static binaries for all platforms to GitHub Releases

## v2 Requirements

### Advanced

- **ADV-01**: User can pipe input via stdin (`cat file.md | convr --from md --to html > out.html`)
- **ADV-02**: Watch mode re-converts files on change (`convr --watch --to webp ./images/`)
- **ADV-03**: Preset profiles (`--preset web`, `--preset print`) apply predefined quality/format settings
- **ADV-04**: Plugin system allows custom converters via `.wasm` or `.dll` drop-in
- **ADV-05**: Optional Tauri GUI for drag-and-drop batch conversion

### Images (deferred)

- **IMG-06**: TIFF multipage — export each page as a separate image
- **IMG-07**: GIF → MP4/WebM animated format conversion
- **IMG-08**: Batch rename with output filename patterns (`--stem "{name}-converted"`)

### Documents (deferred)

- **DOCS-07**: ODT (OpenDocument text) read/write support

## Out of Scope

| Feature | Reason |
|---------|--------|
| YAML comments/anchors preservation | Lost through serde serialization — no workaround |
| SVG → HEIC | Requires libheif write support; deferred |
| Cross-category conversion (image ↔ document) | Intentionally unsupported by design |
| Cloud formats (Google Docs, Notion, etc.) | Local-only tool — no network access |
| Windows MSI / winget package | Static binary via GitHub Releases is sufficient for v1 |

## Traceability

| Requirement | Phase | Status |
|-------------|-------|--------|
| CLIB-01 | Phase 1 | Pending |
| CLIB-02 | Phase 1 | Pending |
| CLIB-03 | Phase 1 | Pending |
| DOCS-01 | Phase 2 | Pending |
| DOCS-02 | Phase 2 | Pending |
| DOCS-03 | Phase 2 | Pending |
| DOCS-04 | Phase 2 | Pending |
| DOCS-05 | Phase 2 | Pending |
| DOCS-06 | Phase 2 | Pending |
| IMG-01 | Phase 3 | Pending |
| IMG-02 | Phase 3 | Pending |
| IMG-03 | Phase 3 | Pending |
| IMG-04 | Phase 3 | Pending |
| IMG-05 | Phase 3 | Pending |
| UX-01 | Phase 4 | Pending |
| UX-02 | Phase 4 | Pending |
| UX-03 | Phase 4 | Pending |
| UX-04 | Phase 4 | Pending |
| UX-05 | Phase 4 | Pending |
| UX-06 | Phase 4 | Pending |

**Coverage:**
- v1 requirements: 20 total
- Mapped to phases: 20
- Unmapped: 0 ✓

---
*Requirements defined: 2026-03-20*
*Last updated: 2026-03-20 after initial definition*
