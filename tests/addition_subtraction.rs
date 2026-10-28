use std::process::Command;

fn run(args: &[&str]) -> (bool, String, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_calculator"))
        .args(args)
        .output()
        .expect("failed to run calculator binary");
    (
        output.status.success(),
        String::from_utf8(output.stdout).unwrap(),
        String::from_utf8(output.stderr).unwrap(),
    )
}

#[test]
fn adds_integers() {
    assert_eq!(run(&["3+2"]), (true, "5\n".into(), String::new()));
}

#[test]
fn subtracts_integers() {
    assert_eq!(run(&["10-7"]), (true, "3\n".into(), String::new()));
}

#[test]
fn prints_real_result() {
    assert_eq!(run(&["1.5+2"]), (true, "3.5\n".into(), String::new()));
}

#[test]
fn prints_integer_when_real_operands_sum_to_integer() {
    assert_eq!(run(&["1.5+1.5"]), (true, "3\n".into(), String::new()));
}

#[test]
fn accepts_whitespace_split_across_arguments() {
    assert_eq!(run(&["3", "-", "5"]), (true, "-2\n".into(), String::new()));
}

#[test]
fn reports_invalid_expression() {
    let (success, stdout, stderr) = run(&["3+"]);
    assert!(!success);
    assert!(stdout.is_empty());
    assert!(!stderr.is_empty());
}
