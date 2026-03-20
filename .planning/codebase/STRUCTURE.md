# Codebase Structure

**Analysis Date:** 2026-03-20

## Directory Layout

```
convr/
├── src/
│   ├── main.rs              # CLI entry point (clap, arg parsing, delegates to batch)
│   ├── formats.rs           # Format and Category enums — central format registry
│   ├── detect.rs            # Source format detection (extension + magic bytes)
│   ├── convert.rs           # Category-level conversion dispatch
│   ├── batch.rs             # Glob expansion, parallel execution, progress bar
│   └── converters/
│       ├── data.rs          # Data format converters (JSON, YAML, TOML, CSV, XML)
│       ├── document.rs      # Document converters (Markdown, HTML, PlainText)
│       └── image.rs         # Image converters (raster, SVG, AVIF, PNG optimization)
├── Cargo.toml               # Manifest, dependencies, optional feature flags
├── Cargo.lock               # Locked dependency versions
├── README.md                # Project overview, usage, architecture notes
└── ROADMAP.md               # 5-phase development plan
```

## Directory Purposes

**`src/`:**
- Purpose: All application source code
- Contains: Flat modules plus one inline submodule directory
- Key files: `main.rs` (entry), `formats.rs` (central registry)

**`src/converters/`:**
- Purpose: One module per format category; all conversion implementation lives here
- Contains: `data.rs`, `document.rs`, `image.rs`
- Declared in `main.rs` as: `mod converters { pub mod data; pub mod document; pub mod image; }`

**`target/`:**
- Purpose: Cargo build output
- Generated: Yes
- Committed: No (in `.gitignore`)

## Key File Locations

**Entry Point:**
- `src/main.rs`: Defines `Cli` struct with `clap::Parser`, validates target format, calls `batch::run()`

**Format Registry:**
- `src/formats.rs`: `Format` enum (19 variants), `Category` enum, all extension/name/category mappings

**Detection:**
- `src/detect.rs`: `pub fn detect(path: &Path) -> Option<Format>` — extension-first, magic-bytes fallback via `infer`

**Dispatch:**
- `src/convert.rs`: `pub fn convert(input, src, output, target) -> Result<()>` — routes by category pair

**Batch Runner:**
- `src/batch.rs`: `pub fn expand_inputs()`, `pub fn run()`, private `resolve_output()`

**Category Converters:**
- `src/converters/image.rs`: Raster conversions, SVG rasterization, AVIF encoding, PNG optimization
- `src/converters/data.rs`: JSON/YAML/TOML/CSV/XML via `serde_json::Value` intermediate
- `src/converters/document.rs`: Markdown→HTML, HTML→Text, Markdown→Text, Text→HTML

**Build Manifest:**
- `Cargo.toml`: Dependencies and three optional feature flags (`vips`, `mupdf`, `heif`) for future native library integrations

## Naming Conventions

**Files:**
- `snake_case.rs` for all source files (e.g., `detect.rs`, `batch.rs`)
- Module directory uses `snake_case/` (e.g., `converters/`)

**Modules:**
- Flat modules declared with `mod name;` in `main.rs`
- Submodules declared inline: `mod converters { pub mod data; pub mod document; pub mod image; }`

**Functions:**
- `snake_case` throughout
- Public API of each converter module is a single function named `convert`
- Private helpers named after their transformation: `svg_to_raster`, `to_avif`, `csv_to_json`, `json_to_csv`, `md_to_html`, `html_to_text`, etc.

**Types:**
- `PascalCase` for enums and structs: `Format`, `Category`, `Cli`
- Enum variants: `PascalCase` (e.g., `Format::Jpeg`, `Category::Image`)

## Where to Add New Code

**New format support (extending an existing category):**
1. Add a variant to `Format` in `src/formats.rs`
2. Add extension mappings in `from_extension()` and `extension()` in `src/formats.rs`
3. Add the variant to the correct arm in `category()` in `src/formats.rs`
4. Add conversion logic in the corresponding `src/converters/{category}.rs`

**New format category (e.g., Audio):**
1. Add a `Category::Audio` variant in `src/formats.rs`
2. Add `Format` variants for the new formats in `src/formats.rs`
3. Create `src/converters/audio.rs` with a `pub fn convert(input, src, output, target) -> Result<()>`
4. Declare `pub mod audio;` inside the `mod converters` block in `src/main.rs`
5. Add a `(Category::Audio, Category::Audio)` match arm in `src/convert.rs`

**New CLI flags:**
- Add fields to the `Cli` struct in `src/main.rs` and thread them through `batch::run()`

**Utilities / shared helpers:**
- If broadly shared, add a new `src/utils.rs` module and declare `mod utils;` in `src/main.rs`
- If category-specific, add private functions directly in the relevant `src/converters/*.rs` file

## Special Directories

**`target/`:**
- Purpose: Cargo build artifacts, debug and release binaries, dependency caches
- Generated: Yes (by `cargo build`)
- Committed: No

**`.planning/`:**
- Purpose: GSD planning documents (codebase analysis, phase plans)
- Generated: By GSD tooling
- Committed: Yes (planning artifacts tracked in git)

---

*Structure analysis: 2026-03-20*
