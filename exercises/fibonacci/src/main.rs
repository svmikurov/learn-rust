use std::io;

fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let (mut a, mut b) = (0, 1);
            for _ in 2..=n {
                (a, b) = (b, a + b);
            }
            b
        }
    }
}

fn main() {
    println!("Generating the nth Fibonacci number");
    println!("Enter n: ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: u32 = input
        .trim()
        .parse()
        .expect("Please enter a valid positive number");

    let result = fibonacci(n);

    println!("The {} Fibonacci number is: {}", n, result);
}
