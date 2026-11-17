//! # 05 · Structs, methods & associated functions
//!
//! ▶ Run: `cargo run --bin q05_structs`
//! 🎯 Goal: fill in the `Rectangle` implementation.
//!
//! HINT 1 ─ `&self` borrows the struct; use it for read-only methods.
//! HINT 2 ─ An associated function with no `self` is called like
//!          `Rectangle::square(4)` and may return `Self`.
//! HINT 3 ─ To build a value, write `Rectangle { width: ..., height: ... }`
//!          or (inside the impl) `Self { ... }`.
//! HINT 4 ─ `{square:?}` prints the derived Debug form.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        // TODO: a square has equal width and height.
        todo!("build a square")
    }

    fn area(&self) -> u32 {
        // TODO
        0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        // TODO: true when `other` fits inside `self`.
        false
    }
}

fn main() {
    let rect = Rectangle { width: 8, height: 5 };
    assert_eq!(rect.area(), 40);

    let square = Rectangle::square(4);
    assert_eq!(format!("{square:?}"), "Rectangle { width: 4, height: 4 }");
    assert_eq!(square.area(), 16);

    assert!(rect.can_hold(&square));
    assert!(!square.can_hold(&rect));

    println!("\n✅ 05 — structs: passed!");
}
