# fledge-plugin-hangman

A [fledge](https://github.com/CorvidLabs/fledge) plugin that plays hangman with identifiers from your codebase.

It scans your project for structs, functions, traits, constants, classes, and other named identifiers, picks one at random, and challenges you to guess it letter by letter.

## Install

```bash
fledge plugins install CorvidLabs/fledge-plugin-hangman
```

## Play

```bash
fledge hangman
```

Run it from any project root. It'll scan source files (Rust, TypeScript, Python, Go, Java, Swift, Kotlin, Ruby, C#, C/C++, Zig, Elixir) and pick a random identifier for you to guess.

You get a hint showing what kind of identifier it is and which file it's from.

## Supported Languages

Any file with these extensions: `.rs`, `.ts`, `.js`, `.py`, `.go`, `.java`, `.swift`, `.kt`, `.rb`, `.cs`, `.cpp`, `.c`, `.h`, `.tsx`, `.jsx`, `.zig`, `.ex`, `.exs`

## License

MIT
