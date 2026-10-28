use std::process::Command;

#[test]
fn binary_runs_successfully_and_prints_nothing() {
    let output = Command::new(env!("CARGO_BIN_EXE_calculator"))
        .output()
        .expect("failed to run calculator binary");
    assert!(output.status.success());
    assert!(output.stdout.is_empty());
}
