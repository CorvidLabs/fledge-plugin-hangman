---
module: hangman
version: 2
status: active
files:
  - src/main.rs

db_tables: []
depends_on: []
---

# Hangman

## Purpose

Discover identifiers in a repository's supported source files and provide an interactive hangman game that helps developers explore the codebase.

## Public API

| Surface | Behavior |
|---------|----------|
| hangman | Scan the current project for identifiers and start an interactive guessing round. |
| help | Print usage without starting a game. |

## Invariants

1. Discovery skips dependency, build, vendor, and repository metadata directories.
2. Only supported source-file extensions participate in identifier discovery.
3. Duplicate identifiers are removed case-insensitively.
4. The game preserves underscores as revealed characters and compares guesses case-insensitively.
5. An empty project reports that no identifiers were found and exits non-zero.
6. Help exits successfully without reading interactive input.

## Behavioral Examples

```
Given a repository containing supported source files with named functions and types
When the developer runs `fledge hangman`
Then the plugin selects a discovered identifier and presents its source kind and file as a hint
```

## Error Cases

| Error | When | Behavior |
|-------|------|----------|
| Current directory unavailable | Startup cannot resolve the working directory | Exit with a clear failure. |
| No identifiers found | No supported source contains a matching identifier | Explain the repository precondition and exit non-zero. |
| Input closes | Standard input reaches EOF during a round | End without falsely reporting a successful guess. |

## Dependencies

- Rust 2021
- `rand`, `console`, `walkdir`, and `regex`

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-07-12 | Document existing identifier discovery and interactive game behavior for SpecSync 5 adoption. |
| 2026-07-13 | CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-hangman-fledge-plugin: Adopt SpecSync 5.0.1 and Trust 1.0.0 governance for the Hangman Fledge plugin |
