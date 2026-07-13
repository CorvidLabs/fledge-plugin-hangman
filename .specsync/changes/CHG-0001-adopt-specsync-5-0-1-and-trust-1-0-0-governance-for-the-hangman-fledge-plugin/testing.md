---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-hangman-fledge-plugin
artifact: testing
---

# Testing

Local acceptance requires:

- `fledge lanes run verify`
- `REQ-hangman-004` evidence is provided by the native CLI help and test coverage in the verification lane.
- `specsync check --strict --require-coverage 100 --force`
- `specsync agents status`
- `fledge trust doctor`
- `git diff --check`

Hosted acceptance requires the new `trust` job and the existing CI checks to remain green on their normal runner.
