// Import the standard environment module to access command-line arguments
use std::env;

// The main function, where the program execution starts
fn main() {
    // Collect command-line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();

    // Check if the number of command-line arguments is not equal to 2
    if args.len() != 2 {
        // If not, panic with an error message
        panic!("Please provide the temperature in Fahrenheit as a command-line argument.");
    }

    // Convert the second argument (temperature in Fahrenheit) to a floating-point number (f64)
    let fahrenheit: f64 = match args[1].parse() {
        // If parsing is successful, assign the parsed value to fahrenheit
        Ok(num) => num,
        // If parsing is unsuccessful
        Err(_) => {
            panic!("Please provide a valid number as the temperature in Fahrenheit.");
        }
    };

    // Calculate the temperature in Celsius using the formula: (Fahrenheit - 32) / 1.8
    let celsius = (fahrenheit - 32.0) / 1.8;

    // Print the converted temperature in both Fahrenheit and Celsius
    println!("{}°F is {}°C", fahrenheit, celsius);
}
