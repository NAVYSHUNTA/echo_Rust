use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

// ヘルパー関数
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected: String = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_on_newline() -> TestResult {
    run(&["hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_on_newline() -> TestResult {
    run(&["-n", "hello", "there"], "tests/expected/hello2.n.txt")
}

// テキストが入力されなかった場合にエラーとなるかのテスト
#[test]
fn dies_no_args() -> TestResult {
    Command::cargo_bin("echor")?
        .assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

// テキストが入力された場合に正常終了するかのテスト
#[test]
fn run1() {
    let mut cmd: Command = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}