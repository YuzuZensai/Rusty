use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if arguments are provided
    if args.len() != 2 {
        eprintln!("Please provide a number as an argument");
        std::process::exit(1);
    }

    // Check if the argument is a number and is positive and store it in n
    let n = match args[1].parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Please provide a positive number as an argument");
            std::process::exit(1);
        }
    };

    // Print the triangle pattern
    for i in 1..=n {
        for _ in 1..=i {
            print!("*");
        }
        println!();
    }
}

// Personally I prefer this code below since it is more simple and readable, but you might not understand some of the syntax / functions used here yet
/*
fn main() {
    let n = match
        env
            ::args()
            .nth(1)
            .and_then(|s| s.parse().ok())
    {
        Some(n) => n,
        None => {
            eprintln!("Please provide a positive number as an argument");
            std::process::exit(1);
        }
    };

    for i in 1..=n {
        println!("{}", "*".repeat(i));
    }
}
*/
