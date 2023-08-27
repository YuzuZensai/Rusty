use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn test_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("circle_circumference")?;
    // Assert that the program should fail when no arguments
    cmd.assert().failure();
    Ok(())
}

#[test]
fn test_calc() -> TestResult {
    let expected = "Circumference: 6.2832\n";
    let mut cmd = Command::cargo_bin("circle_circumference")?;
    // Assert that the program should output expected results
    cmd.arg("1").assert().success().stdout(expected);
    Ok(())
}
