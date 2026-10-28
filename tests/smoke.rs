use std::process::Command;

#[test]
fn binary_runs_successfully() {
    let status = Command::new(env!("CARGO_BIN_EXE_calculator"))
        .status()
        .expect("failed to run calculator binary");
    assert!(status.success());
}
