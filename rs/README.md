# 🦀 Rust Quiz Sandbox

A tiny collection of beginner Rust puzzles. Each file in `src/bin/` is one
self-contained program with a handful of `TODO`s and `assert!` checks. Your job:
make the checks pass until you see the green ✅.

No dependencies, no setup beyond a Rust toolchain.

```bash
cd ~/Projects/playgrounds/rs
cargo run --bin q01_variables
```

> The first puzzle intentionally does **not** compile. That is the puzzle.

## How it works

- Every quiz lives in `src/bin/qNN_topic.rs`.
- Hints are comments at the top of each file — read them only when you're stuck.
- Some quizzes are compile-error puzzles (ownership, borrowing, lifetimes),
  others fail an assertion at runtime (logic).
- A quiz is done when it exits successfully and prints `✅`.

Run the whole set:

```bash
./check.sh              # all quizzes, with a summary
./check.sh --fail-fast  # stop at the first failing quiz
```

## The quizzes

| # | Quiz | Topic | What you practise |
|---|------|-------|-------------------|
| 01 | `q01_variables` | Variables | `mut`, shadowing, `const`, type changes |
| 02 | `q02_functions` | Functions | return values, loops, helpers |
| 03 | `q03_ownership` | Ownership | moves, `&str` vs `String`, borrowing |
| 04 | `q04_borrowing` | Borrowing | `&` vs `&mut`, non-lexical lifetimes |
| 05 | `q05_structs` | Structs | `impl`, `&self`, constructors, `Debug` |
| 06 | `q06_enums` | Enums | data-carrying variants, `match`, `Option` |
| 07 | `q07_strings` | Strings | slices, `chars()`, `split_whitespace` |
| 08 | `q08_collections` | Collections | `Vec`, `HashMap`, entry API, `retain` |
| 09 | `q09_iterators` | Iterators | `filter`/`map`/`sum`/`enumerate` |
| 10 | `q10_errors` | Errors | `Result`, `Option`, `?`, `ok_or` |
| 11 | `q11_traits` | Generics | trait bounds, default methods, `PartialOrd` |
| 12 | `q12_lifetimes` | Lifetimes | borrow relationships, structs with `&str` |
| 13 | `q13_patterns` | Patterns | `match`, ranges, guards, destructuring |
| 14 | `q14_closures` | Closures | `Fn`/`FnMut`, capture, `move` |

Suggested order: 01 → 14. They build on each other, but each file stands alone.

## Rules of the game

1. Try the compiler's error message first. It usually names the exact line.
2. Prefer the smallest fix. Don't delete the `assert!` lines.
3. Don't add `#[allow(...)]` to silence a problem — fix the problem.
4. `cargo clippy` is a great second opinion.
5. Cloning everything technically works, but borrowing is the point of 03/04.

## If you're stuck

```bash
rustc --explain E0382   # use of moved value
rustc --explain E0502   # conflicting borrows
rustc --explain E0106   # missing lifetime specifier
```

Compiler error codes name the lesson: `E0382` = ownership, `E0502`/`E0499` =
borrowing, `E0106` = lifetimes, `E0004` = non-exhaustive match.

## Bonus round

[`QUIZ.md`](QUIZ.md) has 30 quick theory questions with hidden answers. Do it
after the puzzles, or whenever you want a theory check.
