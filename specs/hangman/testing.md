---
spec: hangman.spec.md
---

## Test Plan

### Integration Tests

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `cargo build --release`
- Verify `./target/release/fledge-hangman --help` exits successfully.
- Verify an empty temporary project reports no identifiers and exits non-zero.
