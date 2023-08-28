// Import the necessary modules and types
use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

// Define a test function for handling temperature conversion from Fahrenheit to Celsius
#[test]
fn test_temperature_conversion_f_to_c() -> TestResult {
    // Define the expected output for the given temperature in Fahrenheit
    let expected = "32°F is 0°C\n";

    // Create a command to run the binary and pass the argument "32" (Fahrenheit)
    let mut cmd = Command::cargo_bin("temperature_f_to_c").unwrap();

    // Assert that the command runs successfully, produces the expected stdout output
    cmd.arg("32").assert().success().stdout(expected);

    // Indicate that the test has passed
    Ok(())
}

// Define a test function for handling missing temperature argument
#[test]
fn test_missing_temperature() -> TestResult {
    // Create a command to run the binary without any arguments
    let mut cmd = Command::cargo_bin("temperature_f_to_c").unwrap();

    // Assert that the command fails (exits with an error)
    cmd.assert().failure();

    // Indicate that the test has passed
    Ok(())
}

// Define a test function for handling invalid temperature input
#[test]
fn test_invalid_temperature() -> TestResult {
    // Create a command to run the binary with a non-numeric temperature argument
    let mut cmd = Command::cargo_bin("temperature_f_to_c").unwrap();

    // Assert that the command fails (exits with an error)
    cmd.arg("abc").assert().failure();

    // Indicate that the test has passed
    Ok(())
}
