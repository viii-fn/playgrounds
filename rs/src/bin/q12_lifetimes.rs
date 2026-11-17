//! # 12 · Lifetimes
//!
//! ▶ Run: `cargo run --bin q12_lifetimes`
//! 🎯 Goal: tell the compiler how long the borrowed values live.
//!
//! HINT 1 ─ A function that returns a borrow needs a lifetime in its signature
//!          when the compiler cannot tell which input it came from.
//! HINT 2 ─ A struct holding a reference also needs a lifetime parameter:
//!          `struct Excerpt<'a> { part: &'a str }`.
//! HINT 3 ─ Lifetimes never change how long a value lives — they only
//!          describe existing relationships between borrows.

// TODO 1: add the missing lifetime annotation(s).
fn longest(x: &str, y: &str) -> &str {
    if x.len() >= y.len() { x } else { y }
}

// TODO 2: make this struct hold a borrowed `&str`.
struct Excerpt {
    part: &str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first = novel.split('.').next().unwrap();
    let excerpt = Excerpt { part: first };
    assert_eq!(excerpt.part, "Call me Ishmael");

    let a = String::from("long string");
    let b = String::from("short");
    assert_eq!(longest(&a, &b), "long string");

    println!("\n✅ 12 — lifetimes: passed!");
}
