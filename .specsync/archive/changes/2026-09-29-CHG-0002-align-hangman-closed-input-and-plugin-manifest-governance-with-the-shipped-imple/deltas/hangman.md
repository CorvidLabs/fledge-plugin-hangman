## MODIFIED

### REQUIREMENT REQ-hangman-003

The interactive round SHALL reveal correct guesses, track incorrect guesses, terminate on a win or loss, and treat
a zero-byte standard-input read as an empty invalid guess rather than a successful guess.

Acceptance Criteria

- Correct and incorrect guesses update the round state.
- A completed word or exhausted attempt count terminates the round.
- EOF does not report a win or another successful guess; the caller must terminate a non-interactive session.

### SPEC SECTION Error Cases

| Error | When | Behavior |
|-------|------|----------|
| Current directory unavailable | Startup cannot resolve the working directory | Exit with a clear failure. |
| No identifiers found | No supported source contains a matching identifier | Explain the repository precondition and exit non-zero. |
| Input closes | Standard input reaches EOF during a round | Treat the zero-byte read as empty invalid input and continue prompting until the caller terminates the process. |

### SPEC SECTION Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-07-12 | Document existing identifier discovery and interactive game behavior for SpecSync 5 adoption. |
| 2 | 2026-07-13 | CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-hangman-fledge-plugin: Adopt SpecSync 5.0.1 and Trust 1.0.0 governance for the Hangman Fledge plugin |
