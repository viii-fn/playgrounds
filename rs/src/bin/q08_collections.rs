//! # 08 · Vec & HashMap
//!
//! ▶ Run: `cargo run --bin q08_collections`
//! 🎯 Goal: count words and remove duplicates.
//!
//! HINT 1 ─ `counts.get(word)` is safe; `counts[word]` panics on a missing key.
//! HINT 2 ─ `map.entry(key).or_insert(0)` inserts a default when missing.
//! HINT 3 ─ The counter is a `&mut usize`, so increment with `*count += 1`.
//! HINT 4 ─ `Vec::retain` keeps elements matching a predicate. A `HashSet`
//!          can remember what you have already seen.

use std::collections::HashMap;

fn word_count(text: &str) -> HashMap<&str, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        // TODO 1: increment the count for `word` without panicking.
        let _ = word;
    }
    counts
}

fn dedupe(values: &mut Vec<i32>) {
    // TODO 2: remove duplicates, keeping the first occurrence and the order.
}

fn main() {
    let text = "the quick the lazy the end";
    let counts = word_count(text);
    assert_eq!(counts["the"], 3);
    assert_eq!(counts["quick"], 1);
    assert_eq!(counts.get("missing"), None);

    let mut values = vec![3, 1, 3, 2, 1, 4];
    dedupe(&mut values);
    assert_eq!(values, vec![3, 1, 2, 4]);

    println!("\n✅ 08 — collections: passed!");
}
