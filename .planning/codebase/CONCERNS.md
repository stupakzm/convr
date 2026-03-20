# Codebase Concerns

**Analysis Date:** 2026-03-20

## Tech Debt

**Copy-paste error in data.rs error message:**
- Issue: The error message in the `target` serialization branch of `data::convert` reads `"Unsupported data source format: {:?}"` but is actually reached when the *target* format is unsupported, not the source. The wrong variable name is interpolated, making error output misleading during debugging.
- Files: `src/converters/data.rs` line 31
- Impact: Confusing error output — users see "source format" error when converting *to* an unsupported data target. Diagnose becomes harder in batch runs.
- Fix approach: Change `src` to `target` in that format string: `bail!("Unsupported data target format: {:?}", target)`.

**AVIF quality and speed are hard-coded constants:**
- Issue: AVIF encoding quality (80.0) and speed (4) in `to_avif` are magic numbers with no user control and no named constants explaining their origin.
- Files: `src/converters/image.rs` lines 67-69
- Impact: Users cannot trade quality for file size or speed. Adding `--quality` / `--speed` flags later will require refactoring the function signature through multiple call layers.
- Fix approach: Define named constants or thread quality/speed through a `ConvertOptions` struct passed down from `batch::run`.

**`json_to_xml` produces no XML escaping:**
- Issue: `write_xml_value` in `data.rs` interpolates string values directly into XML tags with `format!("<{tag}>{s}</{tag}>")`. Characters `<`, `>`, `&`, `"`, and `'` in values are not escaped.
- Files: `src/converters/data.rs` lines 138-158
- Impact: Any data value containing `<`, `>`, or `&` produces malformed XML. Round-tripping such data through `json → xml → json` silently corrupts it.
- Fix approach: Apply XML entity escaping (`&amp;`, `&lt;`, `&gt;`) before inserting string and scalar values into the output buffer.

**`json_to_csv` uses key order from first row only:**
- Issue: CSV column headers are derived exclusively from the first JSON object in the array. Subsequent rows that contain keys absent from the first object are silently dropped; those with extra keys are ignored.
- Files: `src/converters/data.rs` lines 59-76
- Impact: JSON arrays with inconsistently-shaped objects produce silently truncated CSV output. No warning is emitted.
- Fix approach: Collect the union of all keys across all objects before writing the header row, then fill missing values with empty strings.

**`detect.rs` reads the entire file into memory for magic-byte detection:**
- Issue: `std::fs::read(path)` in the fallback branch of `detect::detect` loads the complete file contents. For large image or data files (multi-GB) this causes an OOM spike before conversion even begins.
- Files: `src/detect.rs` lines 13-16
- Impact: Potential OOM on large files during batch runs. The `infer` crate only needs the first 16 bytes to identify most formats.
- Fix approach: Read only the first 16–64 bytes using a fixed-size buffer instead of `fs::read`.

**`svg_to_raster` reads the entire SVG into memory:**
- Issue: `std::fs::read(input)` loads the complete SVG file before parsing. Very large SVGs (procedurally generated, embedded raster data URIs) will consume significant memory.
- Files: `src/converters/image.rs` line 37
- Impact: Memory pressure during parallel batch conversion of large SVGs with `rayon`.
- Fix approach: Stream the file via `BufReader` if the `resvg`/`usvg` API supports it; otherwise document the memory implication.

**`strip_html_tags` is a naive character-state machine:**
- Issue: HTML tag stripping in `document.rs` uses a simple `in_tag` boolean that does not handle: HTML comments (`<!-- -->`), CDATA sections, `<script>` / `<style>` tag content (emits script/CSS text), attribute values containing `>`, or self-closing tags with unusual spacing.
- Files: `src/converters/document.rs` lines 59-71
- Impact: `html_to_text` and `md_to_text` can emit raw JavaScript, CSS, or mangled content. Output quality is unreliable for real-world HTML.
- Fix approach: Use the `scraper` or `html5ever` crate for proper HTML parsing, or at minimum skip `<script>` and `<style>` block content.

**XML parsing ignores element attributes entirely:**
- Issue: `xml_to_json` processes `Event::Start`, `Event::End`, and `Event::Text` events but discards attributes on elements. An element like `<item id="1">` loses the `id="1"` attribute in the JSON output.
- Files: `src/converters/data.rs` lines 87-130
- Impact: XML → JSON is lossy for any XML that relies on attributes (the majority of real XML schemas). Round-tripping is not possible.
- Fix approach: Iterate over `e.attributes()` in the `Event::Start` arm and insert them as `@attr-name` keys in the object map (following the convention used by BadgerFish or similar conventions).

**`resolve_output` silently clobbers the input file when no `--out` is given:**
- Issue: When the target extension matches the source extension (e.g., `convr --to png file.png`), `resolve_output` returns the same path as the input. `img.save(output)?` then overwrites the original file in place with no warning or dry-run option.
- Files: `src/batch.rs` lines 67-74
- Impact: Data loss — the original file is destroyed before the re-encoded result is confirmed good. This is especially destructive during batch runs.
- Fix approach: Detect when `output == input` and either bail with an error, append a suffix (e.g., `-converted`), or require `--out` in same-format conversions.

---

## Known Bugs

**Copy/paste bug — wrong variable in error message:**
- Symptoms: Error reads `"Unsupported data source format: Json"` when converting JSON to an unsupported target format.
- Files: `src/converters/data.rs` line 31
- Trigger: `convr --to pdf data.json` (any data file to a non-data target that reaches the serialization branch)
- Workaround: Read the actual conversion attempted from context; the message direction is reversed.

---

## Security Considerations

**XML parsing via quick-xml has no entity expansion limits:**
- Risk: A maliciously crafted "Billion Laughs"-style XML input could expand deeply nested entity references during parsing and exhaust memory or CPU.
- Files: `src/converters/data.rs` (xml_to_json)
- Current mitigation: None — `quick-xml` does not expand entities by default in its streaming API, which partially mitigates this. However, deeply nested elements can still cause O(n) stack depth via the `stack` vector.
- Recommendations: Cap maximum nesting depth and document that untrusted XML inputs should not be processed.

**No validation of output path against input path ancestry:**
- Risk: A crafted glob input combined with a relative `--out` path could cause output files to overwrite arbitrary locations if the tool is run with elevated privileges or in a scripted pipeline.
- Files: `src/batch.rs` lines 43-44, `src/convert.rs` lines 7-10
- Current mitigation: `create_dir_all` is guarded only by an empty-string check. No canonicalization or path traversal check is performed.
- Recommendations: Canonicalize both input and output paths before writing; warn when output falls outside the working directory or input directory.

---

## Performance Bottlenecks

**Full-file read for magic-byte detection duplicates I/O:**
- Problem: `detect::detect` reads the entire file for the `infer` fallback, then `convert::convert` opens the same file again from scratch.
- Files: `src/detect.rs` lines 13-16, `src/converters/image.rs`, `src/converters/data.rs`
- Cause: Detection and conversion are decoupled with no shared file handle or buffered bytes.
- Improvement path: Read only the first 64 bytes for detection; pass the already-detected format + open file handle to the converter.

**PNG oxipng re-optimization writes and re-reads the file:**
- Problem: After `img.save(output)` in image conversion, `oxipng::optimize` opens the same output file again from disk for optimization. This is two full write-read-write cycles for every PNG output.
- Files: `src/converters/image.rs` lines 18-27
- Cause: The `image` crate does not expose in-memory PNG bytes compatible with `oxipng`'s in-memory API in the current usage.
- Improvement path: Use `oxipng::optimize_from_memory` on the PNG bytes produced by `image` before writing to disk, eliminating one disk round-trip.

**`rayon` parallelism with no concurrency limit:**
- Problem: `files.par_iter().for_each(...)` in `batch::run` processes all files in parallel with the default rayon thread count (number of logical CPUs). For large batches of high-resolution images this means all files are decoded into memory simultaneously.
- Files: `src/batch.rs` line 36
- Cause: No semaphore or chunk size limit is applied.
- Improvement path: Use `rayon::ThreadPoolBuilder` to cap the pool size, or chunk the input and process chunks sequentially to bound peak memory.

---

## Fragile Areas

**`xml_to_json` stack-based parser with no depth limit:**
- Files: `src/converters/data.rs` lines 95-129
- Why fragile: Every `Event::Start` pushes onto `stack` with no bound. A deeply nested XML document will grow the stack unboundedly. An empty or malformed document where `Event::End` fires more than `Event::Start` will silently produce `None` root and surface a generic "Failed to parse XML" error.
- Safe modification: Add a depth counter; bail if it exceeds a reasonable limit (e.g., 512). Add handling for the case where `Event::End` fires on an empty stack.
- Test coverage: Zero — no test files exist anywhere in the project.

**`svg_to_raster` non-PNG path encodes twice:**
- Files: `src/converters/image.rs` lines 49-54
- Why fragile: The `_` arm encodes the `tiny-skia` pixmap to PNG bytes in memory, then decodes those bytes back with the `image` crate, then re-encodes to the target format. Any `encode_png` error surfaces as an opaque failure. Adding new image targets does not require visiting this branch, making it easy to miss that new targets also go through this inefficient double-encode path.
- Safe modification: Add a comment documenting the intentional double-encode; consider routing through AVIF encoding path separately.
- Test coverage: Zero.

**`batch::run` swallows per-file errors and continues:**
- Files: `src/batch.rs` lines 46-53
- Why fragile: Individual file conversion errors are counted but not stored. At the end, only the count is reported — the actual error messages are only visible if the user is watching the progress bar in real time (via `pb.println`). Redirected or piped output loses all per-file error details.
- Safe modification: Collect `(path, error)` pairs into a thread-safe list (e.g., `Mutex<Vec<(PathBuf, String)>>`) and print a summary after `pb.finish_and_clear`.
- Test coverage: Zero.

---

## Test Coverage Gaps

**No tests exist at all:**
- What's not tested: Every module — `formats`, `detect`, `batch`, `convert`, `converters/data`, `converters/image`, `converters/document`.
- Files: All `src/**/*.rs` files contain zero `#[test]` functions or `#[cfg(test)]` modules. No integration test directory (`tests/`) exists.
- Risk: Regressions in any conversion path, the copy-paste bug in `data.rs`, XML/CSV edge cases, and the output-clobbers-input scenario are all undetectable without manual testing.
- Priority: High — the codebase is entirely untested. The XML, CSV, and document converters each have multiple correctness issues that a small test suite would have caught.

**No CI pipeline:**
- What's not tested: Cross-platform build correctness (Windows, Linux, macOS). The ROADMAP lists a GitHub Actions CI as a Phase 4 item.
- Files: No `.github/workflows/` directory exists.
- Risk: Platform-specific path or encoding bugs (especially on Windows with `OsStr` / path separators) go undetected.
- Priority: High — the tool targets three platforms with a single binary goal.

---

## Scaling Limits

**In-memory data conversion:**
- Current capacity: Entire file content is read into a `String` via `fs::read_to_string` before parsing.
- Limit: Multi-GB CSV or JSON files will exhaust available RAM before conversion begins.
- Scaling path: Use streaming parsers (`csv::Reader` already supports streaming from a `Read` impl; `serde_json` has a streaming deserializer). Data conversion would need to be restructured to avoid the intermediate `serde_json::Value` representation for large files.

---

## Dependencies at Risk

**`serde_yaml 0.9` is deprecated:**
- Risk: The `serde_yaml` crate at version 0.9 is deprecated upstream. The maintainer has published `serde_yaml` 0.9 as a final version and recommends migrating to alternatives such as `serde_norway` or `marked-yaml`. Future Rust edition or `serde` API changes may break compilation without a maintained upgrade path.
- Impact: YAML conversion (`json_to_yaml`, `yaml_to_json`) breaks if the crate becomes unmaintained against future Rust toolchain versions.
- Migration plan: Evaluate `serde_norway` (direct fork) or `marked-yaml` as a drop-in replacement. The API surface used here is limited to `serde_yaml::from_str` and `serde_yaml::to_string`, making migration low-risk.

**Optional feature flags (`vips`, `mupdf`, `heif`) are entirely unimplemented:**
- Risk: `Cargo.toml` declares three feature flags that compile to nothing — no conditional code, no `#[cfg(feature = "...")]` gates anywhere in the source. A user enabling `--features vips` gets no additional functionality and no compile-time warning.
- Impact: Misleading to contributors; any work done against these flags has no connection to actual code.
- Files: `Cargo.toml` lines 40-43
- Migration plan: Either add stub `unimplemented!()` modules behind the flags (so the API shape is visible) or remove the flags until the Phase 1 work begins.

---

*Concerns audit: 2026-03-20*
