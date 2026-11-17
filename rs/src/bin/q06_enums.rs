//! # 06 · Enums, match & Option
//!
//! ▶ Run: `cargo run --bin q06_enums`
//! 🎯 Goal: complete the `match` expressions and the Option helper.
//!
//! HINT 1 ─ `match` must be exhaustive: cover every variant.
//! HINT 2 ─ `Option<T>` is either `Some(value)` or `None`.
//! HINT 3 ─ Enums can hold data: `Shape::Circle(r)` or a struct-like variant.
//! HINT 4 ─ Use π ≈ 3.14159 for the circle area.

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // TODO 1: `Quarter` is missing — the compiler will tell you.
    }
}

enum Shape {
    Circle(f64),
    Rect { width: f64, height: f64 },
}

fn area(shape: &Shape) -> f64 {
    // TODO 2: circle = π * r², rectangle = width * height.
    0.0
}

fn first_even(numbers: &[i32]) -> Option<i32> {
    // TODO 3: return the first even number, or `None`.
    None
}

fn main() {
    assert_eq!(value_in_cents(&Coin::Quarter), 25);
    assert_eq!(value_in_cents(&Coin::Penny), 1);
    assert_eq!(value_in_cents(&Coin::Nickel), 5);
    assert_eq!(value_in_cents(&Coin::Dime), 10);

    assert!((area(&Shape::Circle(1.0)) - 3.14159).abs() < 1e-9);
    assert_eq!(area(&Shape::Rect { width: 3.0, height: 2.5 }), 7.5);

    assert_eq!(first_even(&[1, 3, 4, 5]), Some(4));
    assert_eq!(first_even(&[1, 3, 5]), None);

    println!("\n✅ 06 — enums & match: passed!");
}
