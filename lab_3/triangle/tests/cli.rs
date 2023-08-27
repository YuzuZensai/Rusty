use assert_cmd::Command;
use std::error::Error;
use assert_cmd::assert::OutputAssertExt;

type TestResult = Result<(), Box<dyn Error>>;

// Check if no arguments are provided
#[test]
fn no_args() -> TestResult {
    Command::cargo_bin("triangle")
        .unwrap()
        .assert()
        .failure()
        .stderr("Please provide a number as an argument\n");
    Ok(())
}

// Negative number
#[test]
fn negative_number() -> TestResult {
    Command::cargo_bin("triangle")
        .unwrap()
        .args(&["-1"])
        .assert()
        .failure()
        .stderr("Please provide a positive number as an argument\n");
    Ok(())
}

#[test]
fn triangle_1() -> TestResult {
    Command::cargo_bin("triangle").unwrap().args(&["1"]).unwrap().assert().success().stdout("*\n");
    Ok(())
}

#[test]
fn triangle_2() -> TestResult {
    Command::cargo_bin("triangle")
        .unwrap()
        .args(&["2"])
        .unwrap()
        .assert()
        .success()
        .stdout("*\n**\n");
    Ok(())
}

#[test]
fn triangle_3() -> TestResult {
    Command::cargo_bin("triangle")
        .unwrap()
        .args(&["3"])
        .unwrap()
        .assert()
        .success()
        .stdout("*\n**\n***\n");
    Ok(())
}

#[test]
fn triangle_10() -> TestResult {
    Command::cargo_bin("triangle")
        .unwrap()
        .args(&["10"])
        .unwrap()
        .assert()
        .success()
        .stdout("*\n**\n***\n****\n*****\n******\n*******\n********\n*********\n**********\n");
    Ok(())
}
