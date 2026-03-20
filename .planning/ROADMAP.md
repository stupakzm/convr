# Roadmap: convr

## Overview

convr v0.1.0 already converts images and data formats with a working binary. This roadmap expands the tool across four phases: wiring in C library backends that unlock advanced formats, expanding document conversion, broadening image format coverage with quality controls, then shipping a polished UX and automated binary distribution for all three platforms.

## Phases

**Phase Numbering:**
- Integer phases (1, 2, 3): Planned milestone work
- Decimal phases (2.1, 2.2): Urgent insertions (marked with INSERTED)

Decimal phases appear between their surrounding integers in numeric order.

- [ ] **Phase 1: C Library Backends** - Wire libheif, libvips, and MuPDF as statically linked features
- [ ] **Phase 2: Document Conversion** - Full read/write for DOCX, EPUB, HTML-to-PDF, and PDF-to-Markdown
- [ ] **Phase 3: Image Format Coverage** - HEIC, JPEG XL, PSD input; resize and quality flags
- [ ] **Phase 4: UX and Distribution** - Format discovery, shell completions, config file, CI, and release binaries

## Phase Details

### Phase 1: C Library Backends
**Goal**: convr can statically link libheif, libvips, and MuPDF so that advanced format conversion is possible without runtime dependencies
**Depends on**: Nothing (first phase)
**Requirements**: CLIB-01, CLIB-02, CLIB-03
**Success Criteria** (what must be TRUE):
  1. `convr` binary on each platform links libheif statically and the crate builds without errors when the heif feature is enabled
  2. `convr` binary links libvips statically and raster operations route through it when the vips feature is enabled
  3. `convr` binary links MuPDF statically and can open, render, and write a PDF document when the mupdf feature is enabled
  4. All three features can be enabled simultaneously in a single build with no link conflicts
**Plans**: TBD

### Phase 2: Document Conversion
**Goal**: Users can convert between DOCX, EPUB, Markdown, HTML, and PDF in both directions
**Depends on**: Phase 1
**Requirements**: DOCS-01, DOCS-02, DOCS-03, DOCS-04, DOCS-05, DOCS-06
**Success Criteria** (what must be TRUE):
  1. User can run `convr --to html input.docx` and get readable HTML output extracted from the document
  2. User can run `convr --to docx input.md` and get a valid DOCX file that opens in Word/LibreOffice
  3. User can run `convr --to text input.epub` and get the plain text content of an EPUB book
  4. User can run `convr --to epub input.md` and get a valid EPUB file that opens in an e-reader
  5. User can run `convr --to pdf input.html` and get a PDF and `convr --to md input.pdf` and get Markdown with layout hints
**Plans**: TBD

### Phase 3: Image Format Coverage
**Goal**: Users can convert HEIC, JPEG XL, and PSD files, and control output dimensions and quality
**Depends on**: Phase 1
**Requirements**: IMG-01, IMG-02, IMG-03, IMG-04, IMG-05
**Success Criteria** (what must be TRUE):
  1. User can run `convr --to jpeg input.heic` and get a valid JPEG from an iPhone photo
  2. User can run `convr --to jxl input.png` and `convr --to png input.jxl` for round-trip JPEG XL conversion
  3. User can run `convr --to png input.psd` and get a flattened PNG from a Photoshop file
  4. User can pass `--width 800` or `--height 600` and the output image is resized to those dimensions
  5. User can pass `--quality 85` and the output JPEG/WebP/AVIF is encoded at the specified quality level
**Plans**: TBD

### Phase 4: UX and Distribution
**Goal**: convr is discoverable, configurable, and automatically released as static binaries for all platforms
**Depends on**: Phase 2, Phase 3
**Requirements**: UX-01, UX-02, UX-03, UX-04, UX-05, UX-06
**Success Criteria** (what must be TRUE):
  1. User can run `convr --list-formats` and see a table of all supported input and output formats
  2. User can run `convr --info input.heic` and see the detected format and file metadata without converting
  3. Shell completions for bash, zsh, fish, and PowerShell are installable from the release archive
  4. User can create `~/.config/convr/config.toml` with default quality and output directory, and convr respects those defaults
  5. Every push to the repository triggers a CI build and test pass on Windows, Linux, and macOS, and every tagged release uploads static binaries to GitHub Releases
**Plans**: TBD

## Progress

**Execution Order:**
Phases execute in numeric order: 1 → 2 → 3 → 4

| Phase | Plans Complete | Status | Completed |
|-------|----------------|--------|-----------|
| 1. C Library Backends | 0/TBD | Not started | - |
| 2. Document Conversion | 0/TBD | Not started | - |
| 3. Image Format Coverage | 0/TBD | Not started | - |
| 4. UX and Distribution | 0/TBD | Not started | - |
