//! # 13 · Pattern matching
//!
//! ▶ Run: `cargo run --bin q13_patterns`
//! 🎯 Goal: use `match` for FizzBuzz, ranges and destructuring.
//!
//! HINT 1 ─ `match (n % 3, n % 5) { (0, 0) => ..., (0, _) => ..., ... }`
//! HINT 2 ─ Range patterns are inclusive: `1..=9`.
//! HINT 3 ─ A guard adds a condition: `n if n < 0 => ...`.
//! HINT 4 ─ Every `match` needs a catch-all `_` when the patterns are not
//!          exhaustive. `if/else` inside a match arm is allowed too.

fn fizzbuzz(n: u32) -> String {
    // TODO 1: 15 -> "FizzBuzz", 3 -> "Fizz", 5 -> "Buzz",
    //         otherwise the number as a String.
    String::new()
}

fn classify(n: i32) -> &'static str {
    match n {
        0 => "zero",
        // TODO 2: negative -> "negative", 1..=9 -> "single digit",
        //         everything else -> "big".
    }
}

fn main() {
    assert_eq!(fizzbuzz(15), "FizzBuzz");
    assert_eq!(fizzbuzz(3), "Fizz");
    assert_eq!(fizzbuzz(5), "Buzz");
    assert_eq!(fizzbuzz(7), "7");

    assert_eq!(classify(-3), "negative");
    assert_eq!(classify(0), "zero");
    assert_eq!(classify(7), "single digit");
    assert_eq!(classify(42), "big");

    // Bonus: destructuring a tuple.
    let point = (3, -7);
    let (x, y) = point;
    assert_eq!(x, 3);
    assert_eq!(y, -7);

    println!("\n✅ 13 — patterns: passed!");
}
