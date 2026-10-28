use std::process::ExitCode;

fn main() -> ExitCode {
    let Some(expression) = std::env::args().nth(1) else {
        return ExitCode::SUCCESS;
    };
    match evaluate(&expression) {
        // f64's Display omits the fractional part of integer-valued answers.
        Ok(answer) => {
            println!("{answer}");
            ExitCode::SUCCESS
        }
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

type Operation = fn(f64, f64) -> f64;

fn operation(symbol: char) -> Option<Operation> {
    match symbol {
        '+' => Some(|a, b| a + b),
        '-' => Some(|a, b| a - b),
        _ => None,
    }
}

/// Evaluates a binary expression such as "3+2" or "10-7".
fn evaluate(expression: &str) -> Result<f64, String> {
    let expression = expression.trim();
    // Skip the first character so that a leading sign belongs to the left operand.
    let (index, apply) = expression
        .char_indices()
        .skip(1)
        .find_map(|(i, c)| operation(c).map(|op| (i, op)))
        .ok_or_else(|| format!("no operator in {expression:?}"))?;
    let left = parse_operand(&expression[..index])?;
    // Operators are all single-byte ASCII characters.
    let right = parse_operand(&expression[index + 1..])?;
    Ok(apply(left, right))
}

fn parse_operand(text: &str) -> Result<f64, String> {
    let text = text.trim();
    text.parse().map_err(|_| format!("invalid number {text:?}"))
}
