use std::process::{Command, Output};

fn calculator(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_calculator"))
        .args(args)
        .output()
        .expect("failed to run calculator binary")
}

#[test]
fn binary_runs_successfully_and_prints_nothing() {
    let output = calculator(&[]);
    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}

#[test]
fn multiplies_and_divides() {
    let cases = [
        ("3*2", "6\n"),
        ("1.5*3", "4.5\n"),
        ("2.5*2", "5\n"),
        ("100/2", "50\n"),
        ("7/2", "3.5\n"),
        ("3 * 2", "6\n"),
    ];
    for (expression, expected) in cases {
        let output = calculator(&[expression]);
        assert!(output.status.success(), "{expression} failed: {output:?}");
        assert_eq!(
            String::from_utf8(output.stdout).unwrap(),
            expected,
            "{expression}"
        );
    }
}

#[test]
fn division_by_zero_prints_error() {
    let output = calculator(&["5/0"]);
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("division by zero"), "stderr: {stderr}");
}

fn stdout_of(expression: &str) -> String {
    let output = calculator(&[expression]);
    assert!(
        output.status.success(),
        "calculator failed on {expression:?}"
    );
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn adds_integers() {
    assert_eq!(stdout_of("3+2"), "5\n");
}

#[test]
fn subtracts_integers() {
    assert_eq!(stdout_of("10-7"), "3\n");
}

#[test]
fn subtraction_can_go_negative() {
    assert_eq!(stdout_of("2-5"), "-3\n");
}

#[test]
fn prints_real_number_when_answer_is_not_integer() {
    assert_eq!(stdout_of("1.5+2"), "3.5\n");
    assert_eq!(stdout_of("5-0.25"), "4.75\n");
}

#[test]
fn prints_integer_when_real_operands_give_integer_answer() {
    assert_eq!(stdout_of("1.5+2.5"), "4\n");
}

#[test]
fn ignores_whitespace() {
    assert_eq!(stdout_of(" 3 + 2 "), "5\n");
}

#[test]
fn reports_error_for_malformed_expression() {
    let output = calculator(&["3+"]);
    assert!(!output.status.success());
    assert!(!output.stderr.is_empty());
}
