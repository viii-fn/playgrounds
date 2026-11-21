fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    const NAME: &str = "Elvis";
    println!("Hey there, {}", NAME);

    {
        let x = 5;

        let x = x + 1;

        {
            let x = x * 2;
            println!("The value of x is: {x}");
        }

        println!("The value of x is: {x}");
    }
}
