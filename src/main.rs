use std::process::ExitCode;

use calculator::{evaluate, format_number};

fn main() -> ExitCode {
    let expression = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    if expression.trim().is_empty() {
        eprintln!(
            "usage: calculator <expression>  (e.g. \"3+2\", \"10-7\", \"3*2\", or \"100/2\")"
        );
        return ExitCode::SUCCESS;
    }
    match evaluate(&expression) {
        Ok(value) => {
            println!("{}", format_number(value));
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
