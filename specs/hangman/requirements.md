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

The interactive round SHALL reveal correct guesses, track incorrect guesses, and terminate on a win, loss, or closed input.

### REQ-hangman-004

Help SHALL exit successfully without interaction, while an empty project SHALL explain the missing prerequisite and exit non-zero.

Acceptance Criteria
- The native Fledge verification lane passes formatting, clippy, tests, release build, and CLI help smoke.
- Verification does not require an interactive game session.

## Constraints

- Discovery is intentionally heuristic and limited to the configured source extensions and naming patterns.

## Out of Scope

- Parsing full language grammars or indexing dependencies and generated output.
