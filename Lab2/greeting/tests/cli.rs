use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("greeting")?;
    cmd.assert().failure(); // Assert that the program should fails
    Ok(())
}

#[test]
fn test_name() -> TestResult {
    let expected = "Hello, nya!\n";
    let mut cmd = Command::cargo_bin("greeting")?;
    // When arg "nya" is specified the expected output should be "Hello, nya!"
    cmd.arg("nya").assert().success().stdout(expected);
    Ok(())
}

#[test]
fn test_name_with_space() -> TestResult {
    let expected = "Hello, nyan nyaaa!\n";
    let mut cmd = Command::cargo_bin("greeting")?;
    // When arg "nyan" and "nyaaa" is specified the expected output should be "Hello, nyan nyaaa!"
    cmd.args(["nyan", "nyaaa"]).assert().success().stdout(expected);
    Ok(())
}
