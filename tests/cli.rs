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
