// Import the standard environment module to access command-line arguments
use std::env;

// The main function, where the program execution starts
fn main() {
    // Collect command-line arguments into a vector of strings
    let args: Vec<String> = env::args().collect();

    // Create an empty vector to store player names
    let mut player_names: Vec<String> = Vec::new();

    // Loop through the arguments starting from index 1 up to 2 (inclusive)
    for i in 1..=2 {
        // Get the argument at index i, or use "N/A" if it's missing
        let name = args.get(i).unwrap_or(&String::from("N/A")).to_string();

        // Add the name to the player_names vector
        player_names.push(name);
    }

    // Print the names of both players
    println!("Player 1: {}", player_names[0]);
    println!("Player 2: {}", player_names[1]);
}
