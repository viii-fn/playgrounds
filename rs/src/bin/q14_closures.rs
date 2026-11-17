//! # 14 · Closures & higher-order functions
//!
//! ▶ Run: `cargo run --bin q14_closures`
//! 🎯 Goal: finish `apply_twice` and the stateful counter closure.
//!
//! HINT 1 ─ `Fn(i32) -> i32` means "callable many times without mutating".
//! HINT 2 ─ A closure that changes captured state implements `FnMut`.
//! HINT 3 ─ `move` takes ownership of captured variables into the closure.
//! HINT 4 ─ `apply_twice(f, x)` should compute `f(f(x))`.

fn apply_twice<F: Fn(i32) -> i32>(f: F, value: i32) -> i32 {
    // TODO 1: call f twice, feeding the first result into the second call.
    let _ = f;
    value
}

fn main() {
    let double = |n| n * 2;
    assert_eq!(apply_twice(double, 3), 12);

    let add = |a, b| a + b;
    assert_eq!(apply_twice(|n| add(n, 5), 1), 11);

    let mut count = 0;
    // TODO 2: make this closure increment `count` on every call.
    let mut bump = || {};
    bump();
    bump();
    bump();
    assert_eq!(count, 3);

    let greeting = String::from("hello");
    let say = move || println!("{greeting}");
    say();
    // `greeting` was moved into `say`. Uncomment to see the move error:
    // println!("{greeting}");

    println!("\n✅ 14 — closures: passed!");
}
