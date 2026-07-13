---
spec: hangman.spec.md
---

## User Stories

- As a developer, I want a lightweight game based on identifiers from my current codebase.

## Acceptance Criteria

### REQ-hangman-001

The plugin SHALL discover supported function, type, constant, trait, and protocol identifiers while skipping dependency and build directories.

### REQ-hangman-002

Identifier selection SHALL deduplicate names case-insensitively and provide the source kind and relative file as a hint.

### REQ-hangman-003

The interactive round SHALL reveal correct guesses, track incorrect guesses, terminate on a win or loss, and treat
a zero-byte standard-input read as an empty invalid guess rather than a successful guess.

Acceptance Criteria

- Correct and incorrect guesses update the round state.
- A completed word or exhausted attempt count terminates the round.
- EOF does not report a win or another successful guess; the caller must terminate a non-interactive session.

### REQ-hangman-004

Help SHALL exit successfully without interaction, while an empty project SHALL explain the missing prerequisite and exit non-zero.

Acceptance Criteria
- The native Fledge verification lane passes formatting, clippy, tests, release build, and CLI help smoke.
- Verification does not require an interactive game session.

## Constraints

- Discovery is intentionally heuristic and limited to the configured source extensions and naming patterns.

## Out of Scope

- Parsing full language grammars or indexing dependencies and generated output.
