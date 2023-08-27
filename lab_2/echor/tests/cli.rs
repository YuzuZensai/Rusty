use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
type TestResult = Result<(), Box<dyn std::error::Error>>;

// Assert the program to fail and the output should contain the word "USAGE" (Indicating the program is not used correctly and showing help message)
#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echor")?.assert().failure().stderr(predicate::str::contains("USAGE"));
    Ok(())
}

// A helper function to run the program with arguments and compare the output stored in a file
fn run(args: &[&str], expected_file: &str) -> TestResult {
    // Read the file and store it in a string
    let expected = fs::read_to_string(expected_file)?;

    // Run the program with the arguments
    Command::cargo_bin("echor")?.args(args).assert().success().stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
