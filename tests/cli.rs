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
    let stderr = stderr_of("5/0");
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

fn stderr_of(expression: &str) -> String {
    let output = calculator(&[expression]);
    assert!(
        !output.status.success(),
        "calculator succeeded on {expression:?}"
    );
    assert!(output.stdout.is_empty(), "stdout for {expression:?}");
    String::from_utf8(output.stderr).unwrap()
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
    assert!(!stderr_of("3+").is_empty());
}

#[test]
fn accepts_negative_operands() {
    assert_eq!(stdout_of("-2+5"), "3\n");
    assert_eq!(stdout_of("-2-3"), "-5\n");
    assert_eq!(stdout_of("3--2"), "5\n");
    assert_eq!(stdout_of("3+-2"), "1\n");
    assert_eq!(stdout_of("3*-2"), "-6\n");
    assert_eq!(stdout_of("-6/-2"), "3\n");
}

#[test]
fn accepts_exponent_notation_operands() {
    assert_eq!(stdout_of("1e-1+1"), "1.1\n");
    assert_eq!(stdout_of("2.5E+1-5"), "20\n");
}

#[test]
fn raises_to_a_power() {
    assert_eq!(stdout_of("2^3"), "8\n");
    assert_eq!(stdout_of("2^-1"), "0.5\n");
    assert_eq!(stdout_of("2.5^2"), "6.25\n");
    assert_eq!(stdout_of("4^0.5"), "2\n");
    assert_eq!(stdout_of("-2^3"), "-8\n");
    assert_eq!(stdout_of("2 ^ 10"), "1024\n");
}

#[test]
fn non_finite_result_prints_error() {
    for expression in [
        "-4^0.5",
        "0^-1",
        "2^2000",
        "1e308*10",
        "1e308+1e308",
        "1+inf",
    ] {
        assert!(!stderr_of(expression).is_empty(), "{expression}");
    }
}
