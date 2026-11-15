fn main() {
    println!("Hello, world!");

    let x: i32 = 5;
    let _y: i32;

    assert_eq!(x, 5); // Assert eq, for equality
    println!("Success");

    let r: &str = "Mutable text";
    println!("{}", r);
}
