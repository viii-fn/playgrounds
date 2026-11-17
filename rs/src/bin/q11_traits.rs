//! # 11 · Generics & traits
//!
//! ▶ Run: `cargo run --bin q11_traits`
//! 🎯 Goal: make `largest` generic and implement a trait.
//!
//! HINT 1 ─ A generic needs a bound before you can compare values:
//!          `fn largest<T: PartialOrd>(list: &[T]) -> &T`.
//! HINT 2 ─ Return a *reference* so you never move a value out of the slice.
//! HINT 3 ─ Traits can provide default methods; implement only `name`.
//! HINT 4 ─ `format!("robot #{model}")` builds the Robot name.

trait Describe {
    fn name(&self) -> String;

    fn describe(&self) -> String {
        format!("I am {}", self.name())
    }
}

struct Dog;

struct Robot {
    model: u32,
}

impl Describe for Dog {
    fn name(&self) -> String {
        "a dog".to_string()
    }
}

// TODO 1: implement `Describe` for `Robot` so that
//         `Robot { model: 7 }.name()` is "robot #7".

fn largest<T>(list: &[T]) -> &T {
    // TODO 2: add the bound, then return a reference to the biggest element.
    // HINT: `item > largest` works on `&T` when `T: PartialOrd`.
    &list[0]
}

fn main() {
    assert_eq!(Dog.describe(), "I am a dog");
    assert_eq!(Robot { model: 7 }.describe(), "I am robot #7");
    assert_eq!(Robot { model: 7 }.name(), "robot #7");

    assert_eq!(*largest(&[3, 9, 2]), 9);
    assert_eq!(*largest(&["apple", "pear"]), "pear");

    println!("\n✅ 11 — generics & traits: passed!");
}
