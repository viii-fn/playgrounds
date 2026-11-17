//! # 07 · Strings & string slices
//!
//! ▶ Run: `cargo run --bin q07_strings`
//! 🎯 Goal: implement `first_word` and `initials`.
//!
//! HINT 1 ─ A `String` is owned; `&str` is a borrowed view. Functions should
//!          usually take `&str` so they accept both.
//! HINT 2 ─ You cannot index a String by byte (`s[0]` will not compile).
//!          Iterate with `s.chars()` instead.
//! HINT 3 ─ `find(' ')` returns `Option<usize>`; slice with `&s[..i]`.
//! HINT 4 ─ `split_whitespace()` + `chars().next()` is a good combo.

fn first_word(text: &str) -> &str {
    // TODO 1: return everything before the first space,
    //         or the whole string when there is no space.
    text
}

fn initials(name: &str) -> String {
    // TODO 2: "Ada Lovelace" -> "A.L."
    String::new()
}

fn main() {
    assert_eq!(first_word("hello world"), "hello");
    assert_eq!(first_word("rust"), "rust");
    assert_eq!(first_word(" leading"), "");

    assert_eq!(initials("Ada Lovelace"), "A.L.");
    assert_eq!(initials("grace brewster hopper"), "G.B.H.");

    // Bonus: why is this safe? Which value owns the bytes?
    let owned = String::from("hello");
    let borrowed: &str = &owned;
    assert_eq!(first_word(borrowed), "hello");
    assert_eq!(owned, "hello");

    println!("\n✅ 07 — strings: passed!");
}
