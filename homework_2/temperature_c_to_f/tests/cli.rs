// Import the necessary modules and types
use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

// Define a test function for handling temperature conversion
#[test]
fn test_temperature_conversion() -> TestResult {
    // Define the expected output for the given temperature in Celsius
    let expected = "0°C is 32°F\n";

    // Create a command to run the binary and pass the argument "0" (Celsius)
    let mut cmd = Command::cargo_bin("temperature_c_to_f").unwrap();

    // Assert that the command runs successfully, produces the expected stdout output
    cmd.arg("0").assert().success().stdout(expected);

    // Indicate that the test has passed
    Ok(())
}

// Define a test function for handling missing temperature argument
#[test]
fn test_missing_temperature() -> TestResult {
    // Create a command to run the binary without any arguments
    let mut cmd = Command::cargo_bin("temperature_c_to_f").unwrap();

    // Assert that the command fails (exits with an error)
    cmd.assert().failure();

    // Indicate that the test has passed
    Ok(())
}

// Define a test function for handling invalid temperature input
#[test]
fn test_invalid_temperature() -> TestResult {
    // Create a command to run the binary with a non-numeric temperature argument
    let mut cmd = Command::cargo_bin("temperature_c_to_f").unwrap();

    // Assert that the command fails (exits with an error)
    cmd.arg("abc").assert().failure();

    // Indicate that the test has passed
    Ok(())
}
