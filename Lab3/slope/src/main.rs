use std::env;
fn main() {
    // Get all arguments passed to the program
    let args: Vec<String> = env::args().collect();

    // Check if all arguments are present
    if args.len() != 5 {
        eprintln!("Please provide 4 arguments! x1, y1, x2, y2");
        std::process::exit(1);
    }

    // Parse arguments to f64
    let x1 = args[1].parse::<f64>().unwrap();
    let y1 = args[2].parse::<f64>().unwrap();
    let x2 = args[3].parse::<f64>().unwrap();
    let y2 = args[4].parse::<f64>().unwrap();

    // Calculate slope
    let result = slope::calculate_slope(x1, y1, x2, y2);

    // Print result
    println!("The slope of the line is: {}", result);
}
