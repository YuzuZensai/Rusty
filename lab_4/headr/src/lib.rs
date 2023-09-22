use clap::{ App, Arg };
use std::error::Error;
use std::fs::File;
use std::io::{ self, BufRead, BufReader, Read };

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("Yuzu")
        .about("Rust head")
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10")
        )
        .arg(
            Arg::with_name("bytes")
                .short("c")
                .long("bytes")
                .value_name("BYTES")
                .takes_value(true)
                .conflicts_with("lines")
                .help("Number of bytes")
        )
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-")
        )
        .get_matches();

    // Parse the line and byte counts
    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    // Iterate over the files
    for (file_num, filename) in config.files.iter().enumerate() {
        // Open the file
        match open(filename) {
            // If there was an error, print it and continue to the next file
            Err(err) => eprintln!("{}: {}", filename, err),
            // If the file was opened
            Ok(mut file) => {
                // If there is more than one file, print the filename
                if config.files.len() > 1 {
                    println!("{}==> {} <==", if file_num > 0 { "\n" } else { "" }, filename);
                }
                // If the bytes option is set, print the bytes else print the lines
                if let Some(num_bytes) = config.bytes {
                    print_bytes(&mut file, num_bytes)?;
                } else {
                    print_lines(&mut file, config.lines)?;
                }
            }
        }
    }
    Ok(())
}

fn print_bytes(file: &mut Box<dyn BufRead>, num_bytes: usize) -> MyResult<()> {
    // Get a handle to the file
    let mut handle = file.take(num_bytes as u64);
    // Create a buffer to hold the bytes
    let mut buffer = vec![0; num_bytes];
    // Read the bytes into the buffer
    let bytes_read = handle.read(&mut buffer)?;
    // Print the bytes
    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
    Ok(())
}

fn print_lines(file: &mut Box<dyn BufRead>, num_lines: usize) -> MyResult<()> {
    // Create a string to hold the line
    let mut line = String::new();
    // Read the lines
    for _ in 0..num_lines {
        // If the number of bytes read is zero, we are done, stores the line contents in the line variable
        if file.read_line(&mut line)? == 0 {
            break;
        }
        // Print the line and clear the line
        print!("{}", line);
        line.clear();
    }
    Ok(())
}

// Open a file
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))), // If filename is "-", read from stdin
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))), // Otherwise, open the file
    }
}

// Parse a string into a positive integer
fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        // If the parse is OK, check that the number is positive
        Ok(n) => {
            // If the number is positive, return it else return an error
            if n > 0 {
                Ok(n)
            } else {
                Err(From::from(val))
            }
        }
        // If the parse fails, return an error
        _ => Err(From::from(val)),
    }
}

#[test]
fn test_parse_positive_int() {
    // 3 is an OK integer
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // Any string is an error
    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    // A zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}
