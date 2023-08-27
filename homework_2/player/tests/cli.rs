// Import the necessary modules and types
use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

// Define a test function for handling player names
#[test]
fn test_1_player_names() -> TestResult {
    // Define the expected output for the given player names
    let expected = "Player 1: Yuzu\nPlayer 2: N/A\n";

    // Create a command to run the binary and pass player names as arguments
    let mut cmd = Command::cargo_bin("player").unwrap();
    cmd.args(&["Yuzu"]).assert().success().stdout(expected);

    // Indicate that the test has passed
    Ok(())
}

// Define a test function for handling player names
#[test]
fn test_2_player_names() -> TestResult {
    // Define the expected output for the given player names
    let expected = "Player 1: Yuzu\nPlayer 2: Nyan\n";

    // Create a command to run the binary and pass player names as arguments
    let mut cmd = Command::cargo_bin("player").unwrap();
    cmd.args(&["Yuzu", "Nyan"]).assert().success().stdout(expected);

    // Indicate that the test has passed
    Ok(())
}

// Define a test function for handling player names
#[test]
fn test_3_player_names() -> TestResult {
    // Define the expected output for the given player names
    let expected = "Player 1: Yuzu\nPlayer 2: Nyan\n";

    // Create a command to run the binary and pass player names as arguments
    let mut cmd = Command::cargo_bin("player").unwrap();
    cmd.args(&["Yuzu", "Nyan", "Meow"]).assert().success().stdout(expected);

    // Indicate that the test has passed
    Ok(())
}

// Define a test function for handling missing player names
#[test]
fn test_missing_player_names() -> TestResult {
    // Define the expected output when player names are missing
    let expected = "Player 1: N/A\nPlayer 2: N/A\n";

    // Create a command to run the binary without any arguments
    let mut cmd = Command::cargo_bin("player").unwrap();
    cmd.assert().success().stdout(expected);

    // Indicate that the test has passed
    Ok(())
}
