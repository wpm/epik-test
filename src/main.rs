use std::process::ExitCode;

fn main() -> ExitCode {
    let expression = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    if expression.trim().is_empty() {
        eprintln!("usage: calculator <expression>  (e.g. \"3*2\" or \"100/2\")");
        return ExitCode::SUCCESS;
    }
    match calculator::evaluate(&expression) {
        Ok(value) => {
            // f64's Display omits the fractional part of integral values, so "6" not "6.0".
            println!("{value}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
