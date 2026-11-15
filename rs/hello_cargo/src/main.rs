// Remove one line from this code to make it compile
fn main() {
    let mut x: i32 = 1;
    x = 7;

    // Shadowing and re-binding
    x += 3;

    let y: i32 = 4;
    // Shadowing
    let y: &str = "I can also bound to text";

    println!("Success");
}
