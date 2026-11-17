//! # 10 · Option & Result
//!
//! ▶ Run: `cargo run --bin q10_errors`
//! 🎯 Goal: handle failure without panicking.
//!
//! HINT 1 ─ `"42".parse::<u8>()` returns `Result<u8, ParseIntError>`.
//! HINT 2 ─ `?` returns early on `Err` when the function returns `Result`.
//! HINT 3 ─ `ok_or(...)` turns an `Option` into a `Result` with your error.
//! HINT 4 ─ `is_empty()` is a friendly way to check for empty slices.

fn parse_age(input: &str) -> Result<u8, String> {
    // TODO 1: parse the input, and reject ages over 150 with a helpful
    //         message. Must NOT panic on "abc".
    //         The error for an out-of-range age is "age must be 0..=150".
    Ok(0)
}

fn average(numbers: &[i32]) -> Option<f64> {
    // TODO 2: return `None` for an empty slice.
    //         HINT: sum as f64 / len as f64.
    Some(0.0)
}

fn main() -> Result<(), String> {
    assert_eq!(parse_age("42"), Ok(42));
    assert!(parse_age("abc").is_err());
    assert_eq!(parse_age("200"), Err("age must be 0..=150".to_string()));

    assert_eq!(average(&[]), None);
    assert_eq!(average(&[2, 4, 6]), Some(4.0));

    // TODO 3: replace `unwrap()` with `?` (main already returns Result).
    let age = parse_age("30").unwrap();
    println!("age = {age}");

    println!("\n✅ 10 — errors: passed!");
    Ok(())
}
