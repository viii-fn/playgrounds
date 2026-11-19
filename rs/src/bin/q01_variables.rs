//! # 01 · Variables, mutability & shadowing
//!
//! ▶ Run: `cargo run --bin q01_variables`
//! 🎯 Goal: make every check pass.
//!
//! HINT 1 ─ Rust variables are immutable by default; add `mut` when a value
//!          really needs to change.
//! HINT 2 ─ Shadowing (`let x = ...` a second time) creates a *new* variable.
//!          It can even change the type.
//! HINT 3 ─ `const` values must have an explicit type and are inlined at use.

const MAX_HEALTH: u32 = 100;

fn main() {
    // TODO 1: make `score` mutable so the next line compiles.
    let mut score = 0;
    score += 10;
    assert_eq!(score, 10);

    // TODO 2: shadow `lives` so it becomes 2 (spend one life).
    let lives = 3;
    let lives = lives - 1; // <- change me
    assert_eq!(lives, 2);

    // TODO 3: shadow `label` so it becomes the String "42".
    //         (Same name, different type — that is allowed with shadowing.)
    let _label = 42;
    let label = "42";
    assert_eq!(label, "42");

    // TODO 4: start from MAX_HEALTH and take 25 damage.
    let health = 75; // <- change me
    assert_eq!(health, MAX_HEALTH - 25);

    println!("\n✅ 01 — variables & mutability: passed!");
}
