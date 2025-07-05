use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_pipe_input() {
    let mut cmd = Command::cargo_bin("temp-converter").unwrap();
    cmd.write_stdin("10").assert().success()
        .stdout(predicate::str::contains("Input: 10.00 °C"))
        .stdout(predicate::str::contains("Fahrenheit: 50.00 °F"))
        .stdout(predicate::str::contains("Kelvin: 283.15 °K"))
        .stdout(predicate::str::contains("Rankine: 509.67 °R"));
}

#[test]
fn test_cli_input() {
    let mut cmd = Command::cargo_bin("temp-converter").unwrap();
    cmd.arg("100").arg("c").assert().success()
        .stdout(predicate::str::contains("Input: 100.00 °C"))
        .stdout(predicate::str::contains("Fahrenheit: 212.00 °F"));
}

#[test]
fn test_cli_input_with_target() {
    let mut cmd = Command::cargo_bin("temp-converter").unwrap();
    cmd.arg("0").arg("c").arg("-t").arg("f").assert().success()
        .stdout(predicate::str::contains("Input: 0.00 °C"))
        .stdout(predicate::str::contains("Converted: 32.00 °F"));
}

#[test]
fn test_no_args_shows_help() {
    let mut cmd = Command::cargo_bin("temp-converter").unwrap();
    cmd.assert().code(0)
        .stdout(predicate::str::contains("Usage:"))
        .stdout(predicate::str::contains("Options:"));
}
