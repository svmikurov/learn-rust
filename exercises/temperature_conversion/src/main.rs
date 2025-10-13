use std::io;

fn convert_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}


fn main() {
    println!("Temperature Converter");
    println!("Enter temperature in degrees Celsius:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let celsius: f64 = input
        .trim()
        .parse()
        .expect("Input number, please");
    
    let fahrenheit: f64 = convert_to_fahrenheit(celsius);

    println!(
        "{}°C is equal to {}°F",
        celsius,
        fahrenheit,
    );
}