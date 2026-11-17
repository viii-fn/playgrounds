//! # 03 · Ownership: moves & borrows
//!
//! ▶ Run: `cargo run --bin q03_ownership`
//! 🎯 Goal: fix the use-after-move errors. Prefer borrowing over cloning.
//!
//! HINT 1 ─ Passing a `String` by value *moves* it into the function.
//!          Passing `&str` only borrows it.
//! HINT 2 ─ `&names[0]` borrows the first element instead of moving it out.
//! HINT 3 ─ If you really need a copy, `name.clone()` exists — but a borrow
//!          is usually the better answer.

fn shout(text: String) -> String {
    text.to_uppercase()
}

fn main() {
    let name = String::from("rust");
    let loud = shout(name);
    // TODO 1: `name` was moved into `shout`. Change `shout` (and this call)
    //         so it borrows instead of taking ownership.
    println!("{name} is loud as {loud}");
    assert_eq!(loud, "RUST");

    let names = vec![String::from("ada"), String::from("grace")];
    // TODO 2: this tries to move a String out of the vector. Borrow it instead.
    let first = names[0];
    println!("first = {first}");
    assert_eq!(first.as_str(), "ada");
    assert_eq!(names.len(), 2);

    println!("\n✅ 03 — ownership: passed!");
}
