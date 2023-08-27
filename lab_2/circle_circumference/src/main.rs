/*
    You could also use clap library here but it's overkill for
    this simple task
*/

const PI: f32 = 3.1416;

fn main() {
    // Stores all passed arguments in a vector
    let args: Vec<String> = std::env::args().collect();

    // Checks if there are any arguments passed
    if args.len() < 2 {
        // Panic marco will quit the program with error exit code
        panic!("No radius specified");
    }

    // Gets the second argument
    // It's the second argument because the first argument is the executable path
    let x_arg = &args[1];

    // Parse arguments into a vector
    let x: f32 = x_arg.parse().unwrap();

    println!("Circumference: {}", calculate_circumference(x));
}

// c = 2πr
fn calculate_circumference(r: f32) -> f32 {
    return 2.0 * PI * r;
}
