# Architecture

**Analysis Date:** 2026-03-20

## Pattern Overview

**Overall:** Pipeline CLI — linear, category-dispatched conversion pipeline with parallel batch execution.

**Key Characteristics:**
- No global state; all data flows through function arguments
- Format knowledge is centralized in a single enum (`Format`) — all other modules pattern-match against it
- Conversion is dispatched first by category pair (Image→Image, Data→Data, Document→Document), then by specific format within each converter module
- Parallelism is applied at the batch level via `rayon`, not inside individual converters
- All errors propagate as `anyhow::Result` — no custom error types

## Layers

**CLI Layer:**
- Purpose: Parse arguments, validate the target format, hand off to batch runner
- Location: `src/main.rs`
- Contains: `Cli` struct (clap derive), `main()` entry point
- Depends on: `formats`, `batch`
- Used by: Nothing — this is the entry point

**Format Registry:**
- Purpose: Single source of truth for all supported formats and their categories
- Location: `src/formats.rs`
- Contains: `Format` enum (19 variants), `Category` enum (3 variants), extension/name parsing, category classification
- Depends on: Nothing
- Used by: All other modules

**Batch Orchestration:**
- Purpose: Expand glob inputs, drive parallel execution, render progress bar, aggregate errors
- Location: `src/batch.rs`
- Contains: `expand_inputs()`, `run()`, `resolve_output()`
- Depends on: `detect`, `convert`, `formats`
- Used by: `main`

**Format Detection:**
- Purpose: Identify the source format of an input file
- Location: `src/detect.rs`
- Contains: `detect()` — tries file extension first, falls back to magic-bytes via the `infer` crate
- Depends on: `formats`
- Used by: `batch`

**Conversion Dispatch:**
- Purpose: Route a (src_format, target_format) pair to the correct category converter
- Location: `src/convert.rs`
- Contains: `convert()` — matches on `(src.category(), target.category())` and delegates
- Depends on: `converters::data`, `converters::image`, `converters::document`, `formats`
- Used by: `batch`

**Category Converters:**
- Purpose: Implement format-specific conversion logic for one category
- Location: `src/converters/image.rs`, `src/converters/data.rs`, `src/converters/document.rs`
- Contains: One public `convert()` function per module, plus private helpers
- Depends on: `formats`, third-party crates specific to the category
- Used by: `convert`

## Data Flow

**Single File Conversion:**

1. `main` parses CLI args; validates `--to` against `Format::from_name()`
2. `batch::expand_inputs()` resolves glob patterns to a `Vec<PathBuf>`
3. `batch::run()` iterates files in parallel via `rayon::par_iter()`
4. Per file: `detect::detect()` returns `Option<Format>` for the source file (extension → magic bytes fallback)
5. `batch::resolve_output()` builds the output path (same dir or `--out` dir, stem + target extension)
6. `convert::convert()` matches `(src.category(), target.category())` and calls the appropriate category converter
7. Each category converter reads the input file, transforms it, and writes the output file
8. Errors are counted atomically; a non-zero count causes `batch::run()` to return `Err`

**Data Converter Intermediate Representation:**
- All data formats (JSON, YAML, TOML, CSV, XML) are first parsed into a `serde_json::Value` tree
- Serialization then writes from that common intermediate to the target format
- This means every data conversion is a two-step parse→serialize, not direct transcoding

**Image Converter Routing:**
- SVG source → `resvg` rasterizer → pixmap → target encoder
- AVIF target → `ravif` encoder (regardless of source, after loading with `image` crate)
- PNG target → `image` crate save + `oxipng` in-place optimization
- All other raster ↔ raster → `image` crate open + save

## Key Abstractions

**`Format` enum:**
- Purpose: Canonical identity for every supported file format
- Location: `src/formats.rs`
- Methods: `from_extension()`, `from_name()` (delegates to `from_extension`), `extension()`, `category()`
- Pattern: All dispatch logic pattern-matches on `Format` variants; no string comparisons outside this module

**`Category` enum:**
- Purpose: Groups formats into conversion families to gate cross-category attempts
- Location: `src/formats.rs`
- Pattern: `convert::convert()` matches on `(src.category(), target.category())` pairs

**Category converter modules:**
- Purpose: Each module encapsulates all logic for one format family
- Examples: `src/converters/image.rs`, `src/converters/data.rs`, `src/converters/document.rs`
- Pattern: Each exposes exactly one `pub fn convert(input, src, output, target) -> Result<()>`

## Entry Points

**`main()` in `src/main.rs`:**
- Triggers: `convr --to <format> [--out <dir>] <inputs...>` invocation
- Responsibilities: Argument parsing via `clap`, target format validation, delegate to `batch::run()`

## Error Handling

**Strategy:** Propagate with `anyhow::Result` throughout; collect and count errors at the batch level rather than aborting on first failure.

**Patterns:**
- `anyhow::bail!()` used for unsupported format combinations in `convert.rs` and the category converters
- `?` operator propagates I/O and parsing errors from converters up to `batch::run()`
- `batch::run()` catches per-file errors via `match`, increments an `AtomicUsize` counter, and logs to the progress bar without aborting the parallel iterator
- Final error: if `errors > 0`, `batch::run()` returns `Err` with a count summary

## Cross-Cutting Concerns

**Logging:** No structured logger. All output goes through `indicatif` progress bar's `pb.println()` to avoid interleaving with the progress display. Final summary printed with `println!`.

**Validation:** Format validation at CLI parse time only (`Format::from_name`). No schema or content validation of input files before conversion attempt.

**Authentication:** Not applicable — local filesystem tool only.

**Parallelism:** `rayon::par_iter()` in `batch::run()` parallelizes across files. Individual converter functions are single-threaded; some crates (oxipng, ravif, image) use internal threading where noted in `Cargo.toml` feature flags.

---

*Architecture analysis: 2026-03-20*
