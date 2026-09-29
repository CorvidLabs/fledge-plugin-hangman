---
change: CHG-0002-align-hangman-closed-input-and-plugin-manifest-governance-with-the-shipped-imple
artifact: testing
---

# Testing

- REQ-hangman-003: inspect the `read_line` result handling; confirm only `Err` exits the input loop before
  validation and `Ok(0)` reaches the empty-input branch. The crate currently has no automated unit tests, so this
  review correction does not claim direct EOF regression coverage.
- Run `fledge lanes run verify` for formatting, Clippy, native tests, release build, and CLI help smoke.
- Run strict SpecSync at 100% file and LOC coverage and confirm `plugin.toml` is governed by the Hangman spec.
- Confirm the accepted Change Log contains aligned version, date, and description cells for versions 2 and 3.
