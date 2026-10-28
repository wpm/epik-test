use std::process::ExitCode;

use calculator::{evaluate, format_number};

fn main() -> ExitCode {
    let expression = std::env::args().skip(1).collect::<String>();
    if expression.is_empty() {
        eprintln!("usage: calculator <expression>  (e.g. calculator 3+2)");
        return ExitCode::SUCCESS;
    }
    match evaluate(&expression) {
        Ok(value) => {
            println!("{}", format_number(value));
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}
