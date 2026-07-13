---
spec: hangman.spec.md
---

## Context

This Rust plugin turns identifiers already present in a developer's project into a terminal game without requiring language servers or external services.

## Related Modules

- Fledge plugin dispatch through `plugin.toml`.
- Repository source files discovered by `walkdir` and regex patterns.

## Design Decisions

- Use heuristic cross-language patterns to keep startup local and dependency-light.
- Deduplicate without case sensitivity so equivalent identifiers do not skew selection.
