# convr

Universal local file converter CLI. Converts documents, images, and data formats with a single command.

```bash
convr --to pdf input.md
convr --to avif *.png --out ./converted/
convr --to yaml data.json
```

## Architecture

Rust CLI + statically linked C libraries. Format detected from file extension + magic bytes.

## Dependencies

### Rust Crates

| Crate | Purpose |
|---|---|
| `clap` | CLI argument parsing |
| `serde` + `serde_json` | JSON serialization/deserialization |
| `serde_yaml` | YAML serialization/deserialization |
| `toml` | TOML serialization/deserialization |
| `csv` | CSV reading/writing |
| `quick-xml` | XML reading/writing |
| `pulldown-cmark` | Markdown → HTML parsing |
| `image` | Pure-Rust raster image decoding/encoding (PNG, JPEG, BMP, GIF, TIFF, WebP, ICO) |
| `resvg` | SVG → raster rendering (better than ImageMagick for SVG) |
| `ravif` | AVIF encoding (pure Rust, rav1e backend) |
| `oxipng` | PNG optimization |
| `infer` | Magic-byte based format detection |
| `glob` | Batch file pattern matching |
| `rayon` | Parallel batch conversion |
| `anyhow` | Error handling |
| `indicatif` | Progress bars for batch ops |

### C Libraries (statically linked)

| Library | Purpose | Formats |
|---|---|---|
| **libvips** | High-performance image processing | 300+ image formats, 3-4x faster than ImageMagick |
| **libheif** | HEIC/HEIF support | HEIC, HEIF (iOS camera format) |
| **MuPDF** | PDF engine | PDF read/write/render, best open-source PDF library |
| **libwebp** | WebP codec (Google's reference impl) | WebP encode/decode |
| **mozjpeg** | Optimized JPEG encoding | JPEG (smaller files than libjpeg) |

### Build Tools Required

| Tool | Purpose |
|---|---|
| `cmake` | Required to build C libraries |
| `pkg-config` | Finds system libraries during build |
| `clang` / LLVM | Required for bindgen (Rust FFI to C) |

## Supported Conversions

### Images
`PNG ↔ JPEG ↔ WebP ↔ AVIF ↔ GIF ↔ BMP ↔ TIFF ↔ ICO`
`SVG → PNG/JPEG/WebP` (via resvg, proper CSS/font rendering)
`HEIC/HEIF → any raster` (via libheif)

### Documents
`Markdown → HTML → PDF`
`HTML → PDF` (via MuPDF)
`PDF → plain text`

### Data Formats
`JSON ↔ YAML ↔ TOML ↔ CSV ↔ XML`

## Installation

```bash
cargo build --release
```

Binary will be at `target/release/convr`.
