---
change: CHG-0002-align-hangman-closed-input-and-plugin-manifest-governance-with-the-shipped-imple
artifact: context
---

# Context

The adopted contract claimed that a round terminates when standard input closes. Rust `read_line` reports EOF as
`Ok(0)`, while the shipped loop exits only on `Err`; it therefore treats EOF as an empty invalid guess and
repeats the prompt. Because this migration must not introduce product code, the canonical contract must describe
that existing boundary rather than certify termination that does not exist.

The public `hangman` command is also declared in `plugin.toml`, but the canonical file mapping and meaningful
path policy governed only `src/main.rs`. This correction brings the manifest under both contract coverage and the
change lifecycle. It also repairs the malformed version-2 Change Log row produced during the original rollout.
