# External Integrations

**Analysis Date:** 2026-03-20

## APIs & External Services

None. `convr` is a fully local, offline CLI tool. It makes no HTTP requests and has no external service dependencies at runtime.

## Data Storage

**Databases:**
- None — no database used

**File Storage:**
- Local filesystem only
  - Reads input files from paths/globs provided as CLI arguments
  - Writes output files to the same directory as input (default) or to `--out <dir>`
  - Output directory is created automatically via `std::fs::create_dir_all` in `src/convert.rs`

**Caching:**
- None

## Authentication & Identity

**Auth Provider:**
- None — no authentication of any kind

## Monitoring & Observability

**Error Tracking:**
- None — errors are printed to stdout via `indicatif`'s `pb.println()` in `src/batch.rs` and propagated as `anyhow::Error`

**Logs:**
- No structured logging framework — runtime feedback is progress bar output and per-file status lines (`ok`, `skip`, `error`) printed to stdout via `indicatif` in `src/batch.rs`

## CI/CD & Deployment

**Hosting:**
- Not yet configured (planned: GitHub Releases static binaries per ROADMAP.md Phase 4)

**CI Pipeline:**
- Not yet configured (planned: GitHub Actions on Windows/Linux/macOS per ROADMAP.md Phase 4)

## Environment Configuration

**Required env vars:**
- None — the tool has zero environment variable requirements

**Secrets location:**
- Not applicable — no secrets or credentials of any kind

## Webhooks & Callbacks

**Incoming:**
- None

**Outgoing:**
- None

## Optional C Library Backends (Feature-Flagged)

These are statically linked at compile time, not runtime integrations. They add capability without introducing network or service dependencies.

| Feature Flag | Library | Source |
|---|---|---|
| `vips` | libvips | System library (via pkg-config) |
| `mupdf` | MuPDF | System library (via pkg-config) |
| `heif` | libheif | System library (via pkg-config) |

Build with: `cargo build --release --features vips,mupdf,heif`

These are currently stub feature flags — the integration code is planned in ROADMAP.md Phase 1 and not yet implemented in `src/`.

## Crates.io Registry

The only external network call in the development workflow is `cargo`'s dependency resolution against the crates.io registry during `cargo build`. This is a standard Rust toolchain behavior, not an application integration.

---

*Integration audit: 2026-03-20*
