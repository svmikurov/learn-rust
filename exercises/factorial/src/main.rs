use std::io;

fn factorial(n: u128) -> u128 {
    match n {
        0 | 1 => 1,
        _ => n * factorial(n - 1),
    }
}

fn main() {
    println!("Generating the nth Factorial number");
    println!("Enter n:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u128 = input
        .trim()
        .parse()
        .expect("Please enter a valid positive number");

    let result = factorial(n);

    println!("The {} Factorial number is: {}", n, result);
}