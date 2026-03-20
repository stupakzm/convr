# Testing Patterns

**Analysis Date:** 2026-03-20

## Test Framework

**Runner:**
- Rust's built-in test harness (`cargo test`) — no external test runner configured
- No `jest.config.*`, `vitest.config.*`, or similar — this is a pure Rust project using `cargo test`
- No additional test framework crates in `Cargo.toml` (no `mockall`, `rstest`, `proptest`, etc.)

**Assertion Library:**
- Standard Rust `assert!`, `assert_eq!`, `assert_ne!` macros (built into the language)
- No third-party assertion crates detected

**Run Commands:**
```bash
cargo test              # Run all tests
cargo test -- --nocapture  # Run with stdout visible
cargo test <name>       # Run tests matching a name filter
cargo clippy            # Lint (no config file; default rules)
cargo fmt --check       # Check formatting without modifying
```

## Test File Organization

**Current State:** No tests exist in the codebase. There are no `#[cfg(test)]` blocks, no `mod tests` modules, no `tests/` integration test directory, and no test files of any kind.

**Rust-standard locations to use when adding tests:**

- Unit tests: inline `#[cfg(test)] mod tests { ... }` at the bottom of each source file
- Integration tests: `tests/` directory at the project root (sibling to `src/`)
- Benchmark tests: `benches/` directory (requires nightly or `criterion` crate)

**Naming convention to follow:**
```
tests/
├── data_conversion.rs     # Integration tests for data format round-trips
├── document_conversion.rs # Integration tests for document conversions
├── image_conversion.rs    # Integration tests for image conversions
└── batch.rs               # Integration tests for glob expansion and batch run
```

## Test Structure

**Standard Rust inline unit test pattern to adopt:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_from_extension_png() {
        assert_eq!(Format::from_extension("png"), Some(Format::Png));
    }

    #[test]
    fn test_format_from_extension_unknown_returns_none() {
        assert_eq!(Format::from_extension("xyz"), None);
    }
}
```

**Standard Rust integration test pattern to adopt (in `tests/` directory):**
```rust
use convr_lib::formats::Format;

#[test]
fn round_trip_json_to_yaml() {
    // arrange
    // act
    // assert
}
```

Note: Integration tests require the crate to expose a library target (`[lib]` in `Cargo.toml`). Currently only a `[[bin]]` target exists — a `[lib]` target must be added before integration tests can import from the crate.

## Mocking

**Framework:** None — no mocking crate is present (`mockall`, `faux`, `double` etc. are all absent from `Cargo.toml`)

**What to mock when tests are added:**
- File system I/O: use `tempfile` crate for real temporary files rather than mocking `std::fs`. This is the idiomatic Rust approach for testing file-producing functions like those in `src/converters/data.rs`, `src/converters/document.rs`, and `src/converters/image.rs`
- Format detection in `src/detect.rs`: test directly using real fixture files in a `tests/fixtures/` directory

**What NOT to mock:**
- `Format::from_extension` and `Format::category` — these are pure functions with no I/O; test directly
- Serialization/deserialization logic in `src/converters/data.rs` — test using in-memory strings

## Fixtures and Factories

**Test Data:** No fixture files exist. A `tests/fixtures/` directory should be created containing minimal real files of each supported format for integration tests.

**Recommended fixture structure:**
```
tests/
└── fixtures/
    ├── sample.json
    ├── sample.yaml
    ├── sample.toml
    ├── sample.csv
    ├── sample.xml
    ├── sample.md
    ├── sample.html
    ├── sample.txt
    ├── sample.png
    ├── sample.jpg
    └── sample.svg
```

**Factory pattern for test data (in-memory):**
```rust
fn sample_json_value() -> serde_json::Value {
    serde_json::json!([{"name": "Alice", "age": 30}])
}
```

**Location:** `tests/fixtures/` for files; helper functions defined locally in each test module or in a shared `tests/common/mod.rs`

## Coverage

**Requirements:** None enforced — no coverage configuration, no CI pipeline with coverage thresholds

**View Coverage:**
```bash
# Using cargo-tarpaulin (must install separately)
cargo install cargo-tarpaulin
cargo tarpaulin --out Html

# Using llvm-cov (requires nightly or rustup component)
cargo install cargo-llvm-cov
cargo llvm-cov --html
```

## Test Types

**Unit Tests:**
- Scope: Individual functions in isolation
- Best candidates right now:
  - `src/formats.rs`: `Format::from_extension`, `Format::from_name`, `Format::extension`, `Format::category` — all pure functions, no I/O
  - `src/detect.rs`: `detect` function — testable with real fixture files
  - `src/batch.rs`: `resolve_output` (private — would need to be made `pub(crate)` or tested via `run`)
  - `src/converters/data.rs`: `csv_to_json`, `json_to_csv`, `xml_to_json`, `json_to_xml` — private helpers, test via public `convert`

**Integration Tests:**
- Scope: Full conversion pipeline from input file to output file
- Approach: write an input fixture to a temp dir, call `convert::convert(...)`, read output file, assert content
- Use `tempfile::TempDir` for isolated temporary directories

**E2E Tests:**
- Not set up. Would involve invoking the `convr` binary directly via `std::process::Command` and checking stdout/stderr/exit code
- Example:
  ```rust
  let output = std::process::Command::new("cargo")
      .args(["run", "--", "--to", "yaml", "tests/fixtures/sample.json"])
      .output()
      .unwrap();
  assert!(output.status.success());
  ```

## Common Patterns

**Async Testing:**
- Not applicable — the codebase is fully synchronous. No `async` functions exist.

**Error Testing:**
```rust
#[test]
fn unsupported_cross_category_returns_error() {
    let result = convert::convert(
        Path::new("tests/fixtures/sample.png"),
        &Format::Png,
        Path::new("/tmp/out.json"),
        &Format::Json,
    );
    assert!(result.is_err());
    let msg = result.unwrap_err().to_string();
    assert!(msg.contains("not supported"));
}
```

**File I/O Testing (recommended pattern):**
```rust
use tempfile::TempDir;

#[test]
fn json_to_yaml_produces_valid_yaml() {
    let tmp = TempDir::new().unwrap();
    let input = tmp.path().join("input.json");
    let output = tmp.path().join("output.yaml");
    std::fs::write(&input, r#"{"key": "value"}"#).unwrap();

    let result = data::convert(&input, &Format::Json, &output, &Format::Yaml);
    assert!(result.is_ok());

    let content = std::fs::read_to_string(&output).unwrap();
    assert!(content.contains("key"));
}
```

**Option Testing (for `Format::from_extension`):**
```rust
#[test]
fn known_extensions_resolve() {
    assert_eq!(Format::from_extension("jpg"), Some(Format::Jpeg));
    assert_eq!(Format::from_extension("JPG"), Some(Format::Jpeg)); // case-insensitive
    assert_eq!(Format::from_extension("yaml"), Some(Format::Yaml));
    assert_eq!(Format::from_extension("yml"), Some(Format::Yaml));
}

#[test]
fn unknown_extension_returns_none() {
    assert_eq!(Format::from_extension("docx"), None);
    assert_eq!(Format::from_extension(""), None);
}
```

## Current Test Coverage Gap

Zero test coverage exists across all source files. The highest-value areas to test first, in priority order:

1. `src/formats.rs` — pure functions, trivial to test, high confidence value
2. `src/converters/data.rs` — data round-trip correctness (JSON→YAML→JSON, CSV round-trip, etc.)
3. `src/convert.rs` — cross-category rejection logic
4. `src/converters/document.rs` — Markdown→HTML, HTML→text correctness
5. `src/detect.rs` — extension detection and magic byte fallback
6. `src/batch.rs` — `expand_inputs` glob behavior, `resolve_output` path logic

---

*Testing analysis: 2026-03-20*
