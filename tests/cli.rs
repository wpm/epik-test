use std::process::{Command, Output};

fn run(expression: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_calculator"))
        .arg(expression)
        .output()
        .expect("failed to run calculator binary")
}

fn stdout(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn assert_prints(expression: &str, expected: &str) {
    let output = run(expression);
    assert!(output.status.success(), "{expression} failed");
    assert_eq!(stdout(&output), expected, "{expression}");
}

#[test]
fn multiplies_integers() {
    assert_prints("3*2", "6");
}

#[test]
fn divides_evenly_to_an_integer() {
    assert_prints("100/2", "50");
}

#[test]
fn divides_unevenly_to_a_real_number() {
    assert_prints("7/2", "3.5");
}

#[test]
fn division_by_zero_prints_an_error() {
    let output = run("1/0");
    assert!(!output.status.success());
    assert!(stdout(&output).is_empty());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.to_lowercase().contains("divide by zero"),
        "stderr: {stderr}"
    );
}
