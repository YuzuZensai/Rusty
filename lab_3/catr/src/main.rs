/*
    This calls the `get_args` function from the `catr` module to parse command line arguments,
    and then calls the `run` function to execute the program logic.

    If an error occurs during argument parsing or program execution, the error message is printed to stderr
    and the program exits with a non-zero status code.
*/
fn main() {
    if let Err(e) = catr::get_args().and_then(catr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
