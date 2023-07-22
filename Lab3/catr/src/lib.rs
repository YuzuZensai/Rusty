use clap::{ App, Arg };

use std::error::Error;
use std::fs::File;
use std::io::{ self, BufRead, BufReader };

// This is a type alias for a Result with a Box that contains a dyn Error trait object
type MyResult<T> = Result<T, Box<dyn Error>>;

/*
    This is a struct named Config with three fields: files, number_lines, and number_nonblank_lines.
    files - vector of strings that represents the files to be read.
    number_lines - indicates whether to number all output lines.
    number_nonblank_lines - indicates whether to number only non-blank output lines.

    The #[derive(Debug)] attribute is used to automatically implement the Debug trait for the Config struct.
    The Debug trait allows us to print the struct in a debug format using the {:?} format specifier.
    This is useful for debugging purposes as it allows us to see the values of the struct's fields.
    Read more about the Debug trait here: https://doc.rust-lang.org/std/fmt/trait.Debug.html
*/
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

/*
    The get_args function is responsible for parsing the command line arguments passed to the program using the clap crate.
    It returns a Result with a Config struct that contains the parsed arguments.

    Read documentation for the clap crate here: https://docs.rs/clap/2.34.0/clap/index.html
*/
pub fn get_args() -> MyResult<Config> {
    //  Creating an instance of the App struct from clap with the program name, version, author, and description
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Yuzu <yuzu@kirameki.cafe>")
        .about("Rust neko (cat)")

        // Defines files arguments that are required and can be passed multiple times with the default value of "-"
        .arg(
            Arg::with_name("files")
                .help("Files to read")
                .multiple(true)
                .default_value("-")
                .required(true)
        )

        // Defines the number-nonblank flag argument that can be passed with the -b or --number-nonblank flags
        .arg(
            Arg::with_name("number-nonblank")
                .short("b")
                .long("number-nonblank")
                .takes_value(false)
                .help("Number non blank output lines")
        )

        // Defines the number flag argument that can be passed with the -n or --number flags
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .takes_value(false)
                .help("Number all output lines")
        )

        // Parses the arguments and returns a clap::ArgMatches struct
        .get_matches();

    // Gets the values of the files argument and unwraps the result
    let files = matches.values_of_lossy("files").unwrap();

    // Check if the number and number-nonblank flags were passed
    let number_lines = matches.is_present("number");
    let number_nonblank_lines = matches.is_present("number-nonblank");

    // Return a Config struct with the parsed arguments
    Ok(Config {
        files,
        number_lines,
        number_nonblank_lines,
    })
}

/*
    The open function is responsible for opening a file for reading.
    It returns a Result with a BufReader

    Since you didn't learn about dynamic types yet, just know that this function can return either a BufReader that reads from stdin or a file
*/
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

/*
    The run function takes a Config struct as an argument and returns a Result with an empty tuple
    It is responsible for reading the files specified in the Config struct and printing their contents to stdout
*/
pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        // For each file, it calls the open function to get a BufReader
        match open(&filename) {
            // If the open function returns an error, it prints the error to stderr
            Err(err) => eprintln!("{}: {}", filename, err),
            // If the file is successfully opened
            Ok(file) => {
                // Declare a variable to keep track of the line number
                let mut line_number = 1;

                for line_result in file.lines() {
                    let line = line_result?;

                    // If number_lines is true or number_nonblank_lines is true and the line is not empty
                    if config.number_lines || (config.number_nonblank_lines && !line.is_empty()) {
                        /*
                            Print the line number and the line
                            The {:>6} format specifier is used add spaces to the left of the line number to make it 6 characters long
                            \t is used to print a tab character

                            Read more about format specifiers here: https://doc.rust-lang.org/std/fmt/
                        */
                        println!("{:>6}\t{}", line_number, line);
                        line_number += 1;
                    } else {
                        // If number_lines and number_nonblank_lines are false then just print the line
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}
