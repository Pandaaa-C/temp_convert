fn main() {
    println!("Hello, welcome to the Temperature Converter!");
    println!("Do you want to enter your Temperature in (C)elcius, (F)ahrenheit or (K)elvin?");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let input = input.trim();
    if input == "C" {
        println!("Enter the temperature in Celcius: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();
        let input: f64 = input.parse().expect("Invalid input");
        println!("The temperature in Fahrenheit is: {}", celcius_to_fahrenheit(input));
        println!("The temperature in Kelvin is: {}", celcius_to_kelvin(input));
    } else if input == "F" {
        println!("Enter the temperature in Fahrenheit: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();
        let input: f64 = input.parse().expect("Invalid input");
        println!("The temperature in Celcius is: {}", fahrenheit_to_celcius(input));
        println!("The temperature in Kelvin is: {}", celcius_to_kelvin(fahrenheit_to_celcius(input)));
    } else if input == "K" {
        println!("Enter the temperature in Kelvin: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();
        let input: f64 = input.parse().expect("Invalid input");
        println!("The temperature in Celcius is: {}", kelvin_to_celcius(input));
        println!("The temperature in Fahrenheit is: {}", celcius_to_fahrenheit(kelvin_to_celcius(input)));
    }
}

fn celcius_to_fahrenheit(celcius: f64) -> f64 {
    celcius * 9.0 / 5.0 + 32.0
}

fn fahrenheit_to_celcius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn celcius_to_kelvin(celcius: f64) -> f64 {
    celcius + 273.15
}

fn kelvin_to_celcius(kelvin: f64) -> f64 {
    kelvin - 273.15
}
