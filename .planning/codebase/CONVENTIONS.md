# Coding Conventions

**Analysis Date:** 2026-03-20

## Naming Patterns

**Files:**
- `snake_case` for all source files: `batch.rs`, `detect.rs`, `formats.rs`, `convert.rs`
- Converter modules grouped in a subdirectory by category: `converters/data.rs`, `converters/document.rs`, `converters/image.rs`

**Functions:**
- `snake_case` for all functions: `expand_inputs`, `resolve_output`, `csv_to_json`, `json_to_csv`, `md_to_html`, `html_to_text`, `svg_to_raster`, `to_avif`
- Private helper functions named with directional convention for format converters: `{src}_to_{target}` (e.g., `csv_to_json`, `md_to_html`, `json_to_xml`)
- Public entry points per module are consistently named `convert` across all converter modules

**Variables:**
- `snake_case` everywhere: `out_dir`, `src_fmt`, `err_count`, `file_name`, `toml_val`
- Short, clear names preferred: `pb` (progress bar), `rdr` (reader), `wtr` (writer)

**Types and Enums:**
- `PascalCase` for types and enum variants: `Format`, `Category`, `Cli`
- Enum variants reflect format names directly: `Format::Png`, `Format::PlainText`, `Format::Avif`
- Struct fields use `snake_case`: `out: Option<PathBuf>`, `inputs: Vec<String>`

**Modules:**
- Flat module names at the top level declared in `src/main.rs` with `mod batch;`, `mod convert;` etc.
- Nested modules declared inline: `mod converters { pub mod data; pub mod document; pub mod image; }`

## Code Style

**Formatting:**
- No `rustfmt.toml` present; standard `rustfmt` defaults assumed
- Alignment used in match arms for readability — extension strings and enum variants are padded with spaces to align vertically (see `src/formats.rs`)
- Single-line enum variant declaration for related groups: `Png, Jpeg, Webp, Avif, Gif, Bmp, Tiff, Ico, Svg,`

**Linting:**
- No `.clippy.toml` present; default Clippy rules apply
- `unwrap()` usage kept to exactly one location: `src/batch.rs:29` (progress bar template), where failure is a programmer error

## Import Organization

**Order:**
1. Crate-internal imports (`use crate::...`)
2. External crate imports (`use anyhow::...`, `use indicatif::...`, `use rayon::...`)
3. Standard library imports (`use std::path::...`, `use std::sync::...`)

**Pattern:**
- Each module imports only what it needs — no wildcard imports except `rayon::prelude::*` in `src/batch.rs`
- Locally scoped `use` statements used inside function bodies when needed (e.g., `quick_xml` imports inside `xml_to_json` in `src/converters/data.rs`)

## Error Handling

**Framework:** `anyhow` crate (`anyhow = "1"`)

**Patterns:**
- All public functions return `Result<()>` or `Result<T>` using `anyhow::Result`
- `?` operator used consistently for error propagation — no manual `match` on `Result` for propagation
- `anyhow::bail!` macro used for early return with a formatted error message (e.g., unsupported conversion paths in `src/convert.rs` and converter modules)
- `anyhow::anyhow!("message")` used with `.ok_or_else(...)` to convert `Option` to `Result` (e.g., `src/main.rs`, `src/converters/data.rs`, `src/converters/image.rs`)
- `anyhow::bail!` preferred over `return Err(anyhow::anyhow!(...))` throughout

**Error message style:**
- Includes context about what failed and the value involved: `"Unknown target format: {}"`, `"Unsupported data source format: {:?}"`
- Cross-category conversion errors name both sides: `"Cross-category conversion {:?} → {:?} not supported"`
- File-level errors printed to progress bar, not stderr: `pb.println(format!("  error {} : {}", ...))`

## Logging

**Framework:** `indicatif` progress bar (`indicatif = "0.17"`) for runtime output; no structured logging library

**Patterns:**
- Progress bar (`ProgressBar`) used in `src/batch.rs` for all per-file status messages
- `pb.println(...)` used inside `par_iter` to print status without breaking the progress bar
- Three status prefixes used: `"  ok    "`, `"  skip  "`, `"  error "` — fixed-width for visual alignment
- `println!` used only for the final summary line in `src/batch.rs`
- No `eprintln!` usage — errors during batch processing are counted and surfaced at the end via `anyhow::bail!`

## Comments

**When to Comment:**
- Single-line comments for non-obvious logic decisions: `// Try extension first`, `// Fall back to magic bytes`, `// Strip tags with a simple regex-free approach`
- Doc comments (`///`) used on public functions when the purpose is not obvious from the signature: `src/detect.rs` uses `/// Detect format from file extension, falling back to magic bytes.`
- Inline comments in match arms only when the logic needs explanation: `// Treat as literal path`
- Section comments used to group related match arms: `// SVG source — use resvg`, `// AVIF target — use ravif`, `// PNG target with oxipng optimization`

**Style:**
- Comments use em dash (`—`) for connective phrases: `// SVG source — use resvg`
- No commented-out code present in the codebase

## Function Design

**Size:** Functions are small and focused. Helper functions extracted immediately when conversion logic is non-trivial (e.g., `csv_to_json`, `json_to_csv`, `xml_to_json`, `json_to_xml` as private helpers in `src/converters/data.rs`)

**Parameters:** Functions receive `&Path` rather than `PathBuf` wherever possible. Format parameters passed as `&Format` references. No mutable parameters except internal state (`out: &mut String`)

**Return Values:** `Result<()>` for side-effectful operations (file writes). `Result<String>` for in-memory transformations. `Option<Format>` for fallible lookups (`from_extension`, `from_name`, `detect`)

## Module Design

**Exports:**
- Modules expose only what is needed: `pub fn convert(...)` is the single public entry point in each converter module
- Helper functions are all private (no `pub` modifier): `csv_to_json`, `json_to_csv`, `md_to_html`, `svg_to_raster`, etc.
- `formats.rs` exposes `Format` and `Category` enums with their `impl` methods fully public

**Barrel Files:** Not used. Modules are declared directly in `src/main.rs` and accessed via explicit `crate::module::item` paths.

## Architecture Patterns

**Intermediate representation pattern:** Data conversion in `src/converters/data.rs` always routes through `serde_json::Value` as a common intermediate — parse input to JSON Value, serialize to target format. This avoids N×M converter implementations.

**Category dispatch:** `src/convert.rs` dispatches to the correct converter module based on `src.category()` and `target.category()` pair — unsupported cross-category pairs return `bail!` immediately.

**Parallel processing:** `rayon::par_iter` in `src/batch.rs` for file-level parallelism. Error counting uses `Arc<AtomicUsize>` to be safe across threads.

---

*Convention analysis: 2026-03-20*
