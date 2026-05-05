use console::style;
use rand::seq::SliceRandom;
use regex::Regex;
use std::collections::HashSet;
use std::io::{self, BufRead, Write};
use std::path::Path;
use walkdir::WalkDir;

const HANGMAN_STAGES: &[&str] = &[
    r#"
  +---+
  |   |
      |
      |
      |
      |
========="#,
    r#"
  +---+
  |   |
  O   |
      |
      |
      |
========="#,
    r#"
  +---+
  |   |
  O   |
  |   |
      |
      |
========="#,
    r#"
  +---+
  |   |
  O   |
 /|   |
      |
      |
========="#,
    r#"
  +---+
  |   |
  O   |
 /|\  |
      |
      |
========="#,
    r#"
  +---+
  |   |
  O   |
 /|\  |
 /    |
      |
========="#,
    r#"
  +---+
  |   |
  O   |
 /|\  |
 / \  |
      |
========="#,
];

fn collect_identifiers(dir: &Path) -> Vec<(String, String)> {
    let mut identifiers: Vec<(String, String)> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    let struct_re =
        Regex::new(r"(?:struct|class|interface|enum|type)\s+([A-Z][A-Za-z0-9_]{3,})").unwrap();
    let fn_re = Regex::new(r"(?:fn|func|def|function)\s+([a-z][a-z0-9_]{3,})").unwrap();
    let const_re = Regex::new(r"(?:const|static|let)\s+([A-Z][A-Z0-9_]{3,})").unwrap();
    let trait_re = Regex::new(r"(?:trait|protocol|impl)\s+([A-Z][A-Za-z0-9_]{3,})").unwrap();

    let skip_dirs: HashSet<&str> = [
        "target",
        "node_modules",
        ".git",
        "dist",
        "build",
        ".fledge",
        "vendor",
    ]
    .into_iter()
    .collect();

    for entry in WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| {
            if e.file_type().is_dir() {
                let name = e.file_name().to_string_lossy();
                !skip_dirs.contains(name.as_ref())
            } else {
                true
            }
        })
        .filter_map(|e| e.ok())
    {
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
        if !matches!(
            ext,
            "rs" | "ts"
                | "js"
                | "py"
                | "go"
                | "java"
                | "swift"
                | "kt"
                | "rb"
                | "cs"
                | "cpp"
                | "c"
                | "h"
                | "tsx"
                | "jsx"
                | "zig"
                | "ex"
                | "exs"
        ) {
            continue;
        }

        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let relative = path.strip_prefix(dir).unwrap_or(path);
        let file_label = relative.to_string_lossy().to_string();

        let patterns: Vec<(&Regex, &str)> = vec![
            (&struct_re, "type"),
            (&fn_re, "function"),
            (&const_re, "constant"),
            (&trait_re, "trait"),
        ];

        for (re, kind) in &patterns {
            for cap in re.captures_iter(&content) {
                let name = cap[1].to_string();
                let lower = name.to_lowercase();
                if lower == "self"
                    || lower == "none"
                    || lower == "some"
                    || lower == "true"
                    || lower == "false"
                    || lower == "main"
                {
                    continue;
                }
                if !seen.contains(&lower) {
                    seen.insert(lower);
                    identifiers.push((name, format!("{} in {}", kind, file_label)));
                }
            }
        }
    }

    identifiers
}

fn display_word(word: &str, guessed: &HashSet<char>) -> String {
    word.chars()
        .map(|c| {
            if c == '_' {
                '_'
            } else if guessed.contains(&c.to_ascii_lowercase()) {
                c
            } else {
                '?'
            }
        })
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        println!("Usage: fledge-hangman");
        println!();
        println!("Play hangman with identifiers from your codebase.");
        println!("Run from any project root with source files.");
        std::process::exit(0);
    }

    let dir = std::env::current_dir().expect("Cannot determine current directory");

    println!();
    println!("{}", style("  Fledge Hangman").cyan().bold());
    println!(
        "{}",
        style("  Guess the identifier from your codebase!").dim()
    );
    println!();

    let identifiers = collect_identifiers(&dir);
    if identifiers.is_empty() {
        println!(
            "{}",
            style("No identifiers found in this project. Try running from a project root.").red()
        );
        std::process::exit(1);
    }

    println!(
        "{}",
        style(format!(
            "  Found {} identifiers to choose from",
            identifiers.len()
        ))
        .dim()
    );

    let mut rng = rand::thread_rng();
    let (word, hint) = identifiers.choose(&mut rng).unwrap();
    let max_wrong = HANGMAN_STAGES.len() - 1;

    let word_lower: String = word.chars().map(|c| c.to_ascii_lowercase()).collect();
    let mut guessed: HashSet<char> = HashSet::new();
    let mut wrong_guesses: Vec<char> = Vec::new();
    let mut wrong_count = 0;

    println!();
    println!("  Hint: {} ({} letters)", style(&hint).yellow(), word.len());
    println!();

    loop {
        println!("{}", style(HANGMAN_STAGES[wrong_count]).red());
        println!();

        let displayed = display_word(word, &guessed);
        println!("  {}", style(&displayed).bold());
        println!();

        if !wrong_guesses.is_empty() {
            println!(
                "  Wrong: {}",
                style(
                    wrong_guesses
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )
                .red()
            );
        }

        let all_guessed = word_lower.chars().all(|c| c == '_' || guessed.contains(&c));

        if all_guessed {
            println!();
            println!(
                "  {} You got it! The answer was: {}",
                style("!!!").green().bold(),
                style(word).green().bold()
            );
            println!("  {} wrong guesses", wrong_count);
            println!();
            break;
        }

        if wrong_count >= max_wrong {
            println!();
            println!(
                "  {} The answer was: {}",
                style("Game over!").red().bold(),
                style(word).yellow().bold()
            );
            println!();
            break;
        }

        print!("  Guess a letter: ");
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).is_err() {
            break;
        }

        let c = match line.trim().chars().next() {
            Some(c) if c.is_ascii_alphabetic() => c.to_ascii_lowercase(),
            _ => {
                println!("  {}", style("Enter a single letter").dim());
                continue;
            }
        };

        if guessed.contains(&c) {
            println!("  {}", style("Already guessed that letter").dim());
            continue;
        }

        guessed.insert(c);

        if word_lower.contains(c) {
            println!("  {}", style("Correct!").green());
        } else {
            wrong_guesses.push(c);
            wrong_count += 1;
            println!("  {}", style("Nope!").red());
        }
        println!();
    }
}
