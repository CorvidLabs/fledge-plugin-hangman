---
id: CHG-0002-align-hangman-closed-input-and-plugin-manifest-governance-with-the-shipped-imple
state: accepted
type: bug_fix
base_commit: e2d83fd307f697a5c172d2563c969fdcde6fa87a
---

# Align Hangman closed-input and plugin manifest governance with the shipped implementation

## Intent

Align Hangman closed-input and plugin manifest governance with the shipped implementation

## Affected Canonical Specs

- `hangman`

## Acceptance Criteria

- The canonical contract no longer claims EOF termination that the shipped loop does not implement; plugin.toml is governed by the Hangman spec and meaningful-path policy; the malformed version-2 changelog row is corrected; native verification and strict SpecSync 100% coverage pass

## No-spec Rationale

Not applicable
