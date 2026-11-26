//! # 02 · Functions & return values
//!
//! ▶ Run: `cargo run --bin q02_functions`
//! 🎯 Goal: implement `gcd` and `is_prime`.
//!
//! HINT 1 ─ The last expression in a block is returned; do not put a
//!          semicolon after it.
//! HINT 2 ─ Euclid's algorithm: while b != 0, replace (a, b) with (b, a % b).
//! HINT 3 ─ `n` is prime when no number from 2 to n/2 divides it evenly.
//!          Bonus: you can stop at the square root.
//! HINT 4 ─ 0 and 1 are not prime.

fn gcd(a: u32, b: u32) -> u32 {
    // TODO: return the greatest common divisor of a and b.
    // (You will need to make the parameters mutable.)
    0
}

fn is_prime(n: u32) -> bool {
    // TODO: return true only if n is prime.
    if n < 2 {
        return false;
    }
    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    true
}

fn main() {
    assert_eq!(gcd(12, 18), 6);
    assert_eq!(gcd(17, 5), 1);
    assert_eq!(gcd(0, 9), 9);

    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(97));
    assert!(!is_prime(100));

    println!("\n✅ 02 — functions: passed!");
}
