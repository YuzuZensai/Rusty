// Import the necessary modules and types
use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

// Define a test function for calculating the circle area
#[test]
fn test_calc() -> TestResult {
    // Define the expected output
    let expected = "The area of the circle with radius 1 is 3.1416\n";

    // Create a command to run the binary and pass the argument "1"
    let mut cmd = Command::cargo_bin("circle_area").unwrap();

    // Assert that the command runs successfully, produces the expected stdout output
    cmd.arg("1").assert().success().stdout(expected);

    // Indicate that the test has passed
    Ok(())
}

// Define a test function for handling no command-line arguments
#[test]
fn test_no_args() -> TestResult {
    // Create a command to run the binary without any arguments
    let mut cmd = Command::cargo_bin("circle_area").unwrap();

    // Assert that the command fails (exits with an error)
    cmd.assert().failure();

    // Indicate that the test has passed
    Ok(())
}

// Define a test function for handling negative radius input
#[test]
fn test_negative_radius() -> TestResult {
    // Create a command to run the binary with a negative radius argument
    let mut cmd = Command::cargo_bin("circle_area").unwrap();

    // Assert that the command fails (exits with an error)
    cmd.arg("-1").assert().failure();

    // Indicate that the test has passed
    Ok(())
}

// Define a test function for handling invalid radius input
#[test]
fn test_invalid_radius() -> TestResult {
    // Create a command to run the binary with a non-numeric radius argument
    let mut cmd = Command::cargo_bin("circle_area").unwrap();

    // Assert that the command fails (exits with an error)
    cmd.arg("a").assert().failure();

    // Indicate that the test has passed
    Ok(())
}
