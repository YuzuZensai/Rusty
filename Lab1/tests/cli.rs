use assert_cmd::Command;

/*
    This test always passes, since it asserts that true is true.
    I commented it out because of that, but you can uncomment it to see that it works.
*/
/*
    #[test]
    fn works() {
        assert!(true);
    }
*/

/*
    This test is a test that checks if the binary "hello" runs successfully.
    It uses the assert() method from the Command to check if the command runs successfully.
    If the command runs successfully, the test passes. If the command fails, the test fails.

    In this case, the test only fails if the binary "hello" does not exist.
    I commented it, because later we will change this check to acutally check if the binary "hello" prints "Hello, world!". 
*/
/*
    #[test]
    fn runs() {
        let mut cmd = Command::cargo_bin("hello").unwrap();
        cmd.assert().success();
    }
*/

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

/*
    Same as the previous test, but this time it checks if the binary "true" runs successfully.
    This test will always pass, since the binary "true" always runs successfully. (Exit code 0)
*/
#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

/*
    This test checks if the binary "false" runs unsuccessfully.
    It uses the Command method from the assert_cmd crate to run the binary "false" and check if it fails.
    If the binary "false" fails, the test passes. If the binary "false" runs successfully, the test fails.
*/
#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

/*
    This test checks if the binary "calc_4_1" runs successfully and prints "The sum is 114" to stdout (from print! marco).
    It uses the Command method from the assert_cmd crate to run the binary "calc_4_1" and check if it runs successfully and prints the expected output.
    If the binary "calc_4_1" runs successfully and prints "The sum is 114" to stdout, the test passes.
    If the binary "calc_4_1" fails or prints something else to stdout, the test fails.
*/
#[test]
fn calc_4_1() {
    let mut cmd = Command::cargo_bin("calc_4_1").unwrap();
    cmd.assert().success().stdout("The sum is 114");
}

/*
    Same as the previous test, but this time it checks if the binary "calc_4_2" runs successfully and prints "34 + 80 = 114" instead.
*/
#[test]
fn calc_4_2() {
    let mut cmd = Command::cargo_bin("calc_4_2").unwrap();
    cmd.assert().success().stdout("34 + 80 = 114");
}

/*
    This test checks if the binary "shout" runs successfully and prints "{abcde}" to stdout (from print! marco).
    It uses the Command method from the assert_cmd crate to run the binary "shout" and check if it runs successfully and prints the expected output.
    If the binary "shout" runs successfully and prints "{abcde}" to stdout, the test passes.
    If the binary "shout" fails or prints something else to stdout, the test fails.
*/
#[test]
fn shout() {
    let mut cmd = Command::cargo_bin("shout").unwrap();
    cmd.assert().success().stdout("{abcde}");
}
