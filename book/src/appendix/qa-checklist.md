# Recipe QA Checklist

Quality assurance checklist for PRS Cookbook recipes.

## Verification Summary

| Category | Items | Status |
|----------|-------|--------|
| Repository Structure | 10 | ✅ |
| Cargo Configuration | 10 | ✅ |
| Core Library | 30 | ✅ |
| Examples A-G | 31 | ✅ |
| Tests & Quality | 27 | ✅ |
| IIUR Compliance | 10 | ✅ |
| Security | 6 | ✅ |
| **Total** | **134** | **✅** |

## Key Checks

### IIUR Compliance

- [x] Examples use `tempfile::tempdir()`
- [x] No global mutable state
- [x] Deterministic RNG seeded by name
- [x] YAML roundtrip is stable
- [x] Same input produces same output
- [x] Pinned dependencies in Cargo.lock
- [x] Works on Linux (primary target)

### Code Quality

- [x] `cargo clippy -- -D warnings` passes
- [x] `cargo fmt --check` passes
- [x] No `unwrap()` in library code
- [x] All public items documented
- [x] Property-based tests exist

### Security

- [x] No hardcoded secrets
- [x] Remote resources require hash
- [x] Permissions follow least privilege
- [x] No unsafe code
- [x] Temp files cleaned up

## Full Checklist

See [QA_CHECKLIST.md](https://github.com/paiml/prs-cookbook/blob/main/QA_CHECKLIST.md) for the complete 134-item checklist.
