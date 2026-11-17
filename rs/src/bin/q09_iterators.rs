//! # 09 · Iterators
//!
//! ▶ Run: `cargo run --bin q09_iterators`
//! 🎯 Goal: replace the loops with iterator chains.
//!
//! HINT 1 ─ `iter()` yields `&T`; `into_iter()` yields `T`.
//! HINT 2 ─ `map` transforms, `filter` keeps, `sum`/`count`/`collect` consume.
//! HINT 3 ─ Iterators are lazy: nothing runs until a consumer asks for values.
//! HINT 4 ─ To sum squares: `.filter(...).map(|n| n * n).sum()`.
//!          You may need `*n` or `.copied()` because `iter()` gives references.

fn sum_of_even_squares(numbers: &[i32]) -> i32 {
    // TODO 1: sum the squares of the even numbers, without a `for` loop.
    // [1, 2, 3, 4] -> 4 + 16 = 20
    0
}

fn count_words_longer_than(text: &str, min_len: usize) -> usize {
    // TODO 2: one iterator chain.
    0
}

fn main() {
    assert_eq!(sum_of_even_squares(&[1, 2, 3, 4]), 20);
    assert_eq!(sum_of_even_squares(&[]), 0);

    assert_eq!(count_words_longer_than("a bb ccc dddd", 2), 2);

    // Bonus (already written): index/value pairs for every value > 10.
    let data = vec![5, 11, 7, 20];
    let picked: Vec<(usize, i32)> = data
        .iter()
        .enumerate()
        .filter(|(_, value)| **value > 10)
        .map(|(index, value)| (index, *value))
        .collect();
    assert_eq!(picked, vec![(1, 11), (3, 20)]);

    println!("\n✅ 09 — iterators: passed!");
}
