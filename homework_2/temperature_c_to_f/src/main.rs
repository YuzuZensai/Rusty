// Import the standard environment module to access command-line arguments
use std::env;

// The main function, where the program execution starts
fn main() {
    // Collect command-line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();

    // Check if the number of command-line arguments is not equal to 2
    if args.len() != 2 {
        // If not, panic with an error message
        panic!("Please provide the temperature in Celsius as a command-line argument.");
    }

    // Convert the second argument (temperature in Celsius) to a floating-point number (f64)
    let celsius: f64 = match args[1].parse() {
        // If parsing is successful, assign the parsed value to celsius
        Ok(num) => num,
        // If parsing is unsuccessful
        Err(_) => {
            panic!("Please provide a valid number as the temperature in Celsius.");
        }
    };

    // Calculate the temperature in Fahrenheit using the formula: (Celsius * 1.8) + 32
    let fahrenheit = celsius * 1.8 + 32.0;

    // Print the converted temperature in both Celsius and Fahrenheit
    println!("{}°C is {}°F", celsius, fahrenheit);
}
