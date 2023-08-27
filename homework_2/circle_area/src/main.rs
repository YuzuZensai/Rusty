// Import the standard environment module to access command-line arguments
use std::env;

// The main function, where the program execution starts
fn main() {
    // Collect command-line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();

    // Check if the number of command-line arguments is not equal to 2
    if args.len() != 2 {
        // If not, panic with an error message
        panic!("Please provide the radius of the circle as a command-line argument.");
    }

    // Convert the second argument (radius) to a floating-point number (f64)
    let radius: f64 = match args[1].parse() {
        // If parsing is successful and the number is non-negative
        Ok(num) if num >= 0.0 => num,
        // If parsing is successful but the number is negative
        Ok(_) => {
            panic!("Please provide a valid non-negative number as the radius of the circle.");
        }
        // If parsing is unsuccessful
        Err(_) => {
            panic!("Please provide a valid number as the radius of the circle.");
        }
    };

    // Calculate the area of the circle using the formula: π * radius * radius
    let area = 3.1416 * radius * radius;

    // Print the calculated area along with the provided radius
    println!("The area of the circle with radius {} is {}", radius, area);
}
