---
change: CHG-0002-align-hangman-closed-input-and-plugin-manifest-governance-with-the-shipped-imple
artifact: requirements
---

# Requirements

### REQ-hangman-003

The interactive round SHALL reveal correct guesses, track incorrect guesses, terminate on a win or loss, and treat
a zero-byte standard-input read as an empty invalid guess rather than a successful guess.

Acceptance Criteria

- Correct and incorrect guesses update the round state.
- A completed word or exhausted attempt count terminates the round.
- EOF does not report a win or another successful guess; the caller must terminate a non-interactive session.
