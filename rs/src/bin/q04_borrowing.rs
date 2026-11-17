//! # 04 · Borrowing rules
//!
//! ▶ Run: `cargo run --bin q04_borrowing`
//! 🎯 Goal: satisfy the borrow checker.
//!
//! The rule: at any moment you may have EITHER one mutable borrow OR any
//! number of immutable borrows — never both at the same time.
//!
//! HINT 1 ─ Use the immutable borrow *before* taking the mutable one, so the
//!          borrow ends earlier (non-lexical lifetimes).
//! HINT 2 ─ `&mut Vec<T>` lets a function grow or change the vector in place.
//! HINT 3 ─ Two `&mut` borrows can coexist as long as they are not alive at
//!          the same time. Declare the second one after the first is done.

fn double_all(values: &mut Vec<i32>) {
    // TODO 1: multiply every element by 2.
    let _ = values;
}

fn main() {
    let mut scores = vec![10, 20, 30];
    let first = &scores[0];
    // TODO 2: reorder these lines so the immutable borrow is finished
    //         before `push` needs a mutable borrow.
    scores.push(40);
    println!("first = {first}");

    double_all(&mut scores);
    assert_eq!(scores, vec![20, 40, 60, 80]);

    let mut text = String::from("hi");
    let a = &mut text;
    let b = &mut text; // TODO 3: declare `b` after `a` is last used.
    a.push('!');
    b.push('?');
    assert_eq!(text, "hi!?");

    println!("\n✅ 04 — borrowing: passed!");
}
