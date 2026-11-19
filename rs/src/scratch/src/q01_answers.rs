fn main() {
    let mut score = 0;
    score += 10;
    assert_eq!(score, 10);

    let lives = 3;
    let lives = lives - 1; // <- change me
    assert_eq!(lives, 2);

    let label = 42;
    let label = "42";
    assert_eq!(label, "42");

    let health = 0; // <- change me
    assert_eq!(health, MAX_HEALTH - 25);

    println!("\n✅ 01 — variables & mutability: passed!");
}
