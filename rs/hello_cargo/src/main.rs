fn main() {
    println!("Hello, world!");

    let x: i32 = 5;
    let _y: i32;

    assert_eq!(x, 5); // Assert eq, for equality
    println!("Success");

    let r: &str = "Mutable text";
    println!("{}", r);
}

// Remove one line from this code to make it compile
fn compile_project {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    let x = x;
    x += 3;

    let y = 4;
    // Shadowing
    let y = "I can also bound to text";

    println!("Success");
}
