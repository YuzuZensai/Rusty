/*
    You could also use clap library here but it's overkill for
    this simple task
*/

fn main() {
    // Stores all passed arguments in a vector
    let args: Vec<String> = std::env::args().collect();

    // Check if name is passed
    // It is 2 here because the first argument is the executable path
    // Second argument is the name
    if args.len() < 2 {
        // Panic marco will quit the program with error exit code
        panic!("Please provide a name");
    }

    // Join all args together, skipping the first argument
    let name = args[1..].join(" ");

    println!("Hello, {}!", name);
}
