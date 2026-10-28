use std::process::Command;

#[test]
fn binary_runs_successfully_and_prints_nothing() {
    let output = run(&[]);
    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}

fn run(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_calculator"))
        .args(args)
        .output()
        .expect("failed to run calculator binary")
}

fn stdout_of(expression: &str) -> String {
    let output = run(&[expression]);
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
    let output = run(&["3+"]);
    assert!(!output.status.success());
    assert!(!output.stderr.is_empty());
}
