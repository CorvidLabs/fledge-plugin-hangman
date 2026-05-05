# fledge-plugin-hangman

```
  +---+
  |   |
  O   |
 /|\  |
 / \  |
      |
=========
```

**Your codebase is trying to kill you.** Can you guess the identifier before the hangman drops?

A [fledge](https://github.com/CorvidLabs/fledge) plugin that turns your own code into a word-guessing challenge. It scans your project for structs, functions, traits, constants, classes, and other named symbols, picks one at random, and dares you to guess it letter by letter.

Perfect for:
- Onboarding onto a new codebase (learn the vocabulary!)
- Taking a quick break without leaving the terminal
- Proving you *really* know your project's internals

## Install

```bash
fledge plugins install CorvidLabs/fledge-plugin-hangman
```

## Play

```bash
fledge hangman
```

Run it from any project root. The plugin walks your source tree, extracts identifiers using language-aware regex patterns, and picks one at random. You get a hint showing what kind of identifier it is (function, type, constant, trait) and which file it lives in.

You have 6 wrong guesses before the hangman is complete. Underscores in identifiers are shown for free (you're welcome).

## Supported Languages

The scanner recognizes source files with these extensions:

| Language | Extensions |
|----------|-----------|
| Rust | `.rs` |
| TypeScript / JavaScript | `.ts`, `.tsx`, `.js`, `.jsx` |
| Python | `.py` |
| Go | `.go` |
| Java | `.java` |
| Swift | `.swift` |
| Kotlin | `.kt` |
| Ruby | `.rb` |
| C# | `.cs` |
| C / C++ | `.c`, `.cpp`, `.h` |
| Zig | `.zig` |
| Elixir | `.ex`, `.exs` |

Directories like `target/`, `node_modules/`, `.git/`, `dist/`, `build/`, `vendor/`, and `.fledge/` are automatically skipped.

## How It Works

1. Walks the file tree from your current directory
2. Matches identifier patterns: `struct`/`class`/`enum`/`type`, `fn`/`func`/`def`/`function`, `const`/`static`/`let` (UPPER_CASE), `trait`/`protocol`/`impl`
3. Filters out common noise (`self`, `None`, `Some`, `true`, `false`, `main`) and short names (< 4 chars)
4. Deduplicates by lowercased form
5. Picks one at random and drops you into the game loop

## Building from Source

```bash
cargo build --release
```

The binary lands at `target/release/fledge-hangman`.

## License

MIT
