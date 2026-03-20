---
phase: 1
slug: c-library-backends
status: draft
nyquist_compliant: false
wave_0_complete: false
created: 2026-03-20
---

# Phase 1 — Validation Strategy

> Per-phase validation contract for feedback sampling during execution.

---

## Test Infrastructure

| Property | Value |
|----------|-------|
| **Framework** | cargo test |
| **Config file** | Cargo.toml (workspace or crate-level) |
| **Quick run command** | `cargo test --features heif,vips,mupdf 2>&1 | tail -20` |
| **Full suite command** | `cargo test --features heif,vips,mupdf -- --include-ignored` |
| **Estimated runtime** | ~60 seconds (first build ~5 min) |

---

## Sampling Rate

- **After every task commit:** Run `cargo test --features heif,vips,mupdf 2>&1 | tail -20`
- **After every plan wave:** Run `cargo test --features heif,vips,mupdf -- --include-ignored`
- **Before `/gsd:verify-work`:** Full suite must be green
- **Max feedback latency:** 60 seconds

---

## Per-Task Verification Map

| Task ID | Plan | Wave | Requirement | Test Type | Automated Command | File Exists | Status |
|---------|------|------|-------------|-----------|-------------------|-------------|--------|
| 1-01-01 | 01 | 0 | CLIB-01 | build | `cargo build --features heif` | ❌ W0 | ⬜ pending |
| 1-01-02 | 01 | 1 | CLIB-01 | integration | `cargo test --features heif heif_` | ❌ W0 | ⬜ pending |
| 1-02-01 | 02 | 1 | CLIB-02 | build | `cargo build --features vips` | ❌ W0 | ⬜ pending |
| 1-02-02 | 02 | 1 | CLIB-02 | integration | `cargo test --features vips vips_` | ❌ W0 | ⬜ pending |
| 1-03-01 | 03 | 2 | CLIB-03 | build | `cargo build --features mupdf` | ❌ W0 | ⬜ pending |
| 1-03-02 | 03 | 2 | CLIB-03 | integration | `cargo test --features mupdf mupdf_` | ❌ W0 | ⬜ pending |
| 1-04-01 | 04 | 3 | CLIB-01,02,03 | integration | `cargo test --features heif,vips,mupdf` | ❌ W0 | ⬜ pending |

*Status: ⬜ pending · ✅ green · ❌ red · ⚠️ flaky*

---

## Wave 0 Requirements

- [ ] `tests/integration/` directory with stub test files for each feature
- [ ] `tests/fixtures/` directory with sample.heic, sample.pdf, sample.png test files
- [ ] Feature flags `heif`, `vips`, `mupdf` declared in `Cargo.toml`
- [ ] `src/lib.rs` or per-module test stubs compiling under each feature flag

*Wave 0 must create test infrastructure before integration tests can run.*

---

## Manual-Only Verifications

| Behavior | Requirement | Why Manual | Test Instructions |
|----------|-------------|------------|-------------------|
| libde265/libaom codecs available at runtime | CLIB-01 | Codec libraries are dynamically linked; presence varies by OS | Run `convr convert sample.heic out.png` and verify exit 0 with non-empty output |
| AGPL-3.0 license propagation review | CLIB-03 | Legal/compliance — not automatable | Review LICENSE file and confirm AGPL-3.0 or Artifex commercial license before distributing binaries with mupdf feature enabled |

---

## Validation Sign-Off

- [ ] All tasks have `<automated>` verify or Wave 0 dependencies
- [ ] Sampling continuity: no 3 consecutive tasks without automated verify
- [ ] Wave 0 covers all MISSING references
- [ ] No watch-mode flags
- [ ] Feedback latency < 60s
- [ ] `nyquist_compliant: true` set in frontmatter

**Approval:** pending
