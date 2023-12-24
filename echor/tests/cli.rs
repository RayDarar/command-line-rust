use std::fs;

use assert_cmd::Command;
use predicates::prelude::*;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn it_panics_without_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage: echor"));

    Ok(())
}

#[test]
fn it_runs_with_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;

    cmd.arg("hello world!").assert().success();

    Ok(())
}

fn run_file_test(filename: &str, args: Vec<&str>) -> TestResult {
    let expected = fs::read_to_string(filename)?;
    let mut cmd = Command::cargo_bin("echor")?;

    cmd.args(args).assert().success().stdout(expected);

    Ok(())
}

#[test]
fn it_is_equal_to_hello1() -> TestResult {
    run_file_test("tests/expected/hello1.txt", vec!["Hello there"])
}

#[test]
fn it_is_equal_to_hello2() -> TestResult {
    run_file_test("tests/expected/hello2.txt", vec!["Hello", "there"])
}

#[test]
fn it_is_equal_to_hello1_with_no_line() -> TestResult {
    run_file_test("tests/expected/hello1.n.txt", vec!["-n", "Hello   there"])
}

#[test]
fn it_is_equal_to_hello2_with_no_line() -> TestResult {
    run_file_test("tests/expected/hello2.n.txt", vec!["-n", "Hello", "there"])
}
