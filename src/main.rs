use std::process::ExitCode;

fn main() -> ExitCode {
    let Some(expression) = std::env::args().nth(1) else {
        return ExitCode::SUCCESS;
    };
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

#[derive(Clone, Copy)]
enum Operator {
    Multiply,
    Divide,
}

impl Operator {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '*' => Some(Self::Multiply),
            '/' => Some(Self::Divide),
            _ => None,
        }
    }

    fn apply(self, left: f64, right: f64) -> Result<f64, String> {
        match self {
            Self::Multiply => Ok(left * right),
            Self::Divide => {
                if right == 0.0 {
                    Err("division by zero".to_string())
                } else {
                    Ok(left / right)
                }
            }
        }
    }
}

/// Evaluates a binary expression such as "3*2" or "100/2".
fn evaluate(expression: &str) -> Result<f64, String> {
    let expression = expression.trim();
    let (index, operator) = expression
        .char_indices()
        .skip(1) // a leading sign belongs to the left operand
        .find_map(|(i, c)| Operator::from_char(c).map(|op| (i, op)))
        .ok_or_else(|| format!("unsupported expression: {expression}"))?;
    let left = parse_operand(&expression[..index])?;
    let right = parse_operand(&expression[index + 1..])?;
    operator.apply(left, right)
}

fn parse_operand(text: &str) -> Result<f64, String> {
    let text = text.trim();
    text.parse()
        .map_err(|_| format!("invalid number: {text:?}"))
}

/// Prints integral values without a fractional part and other values as reals.
fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_negative_left_operand() {
        assert_eq!(evaluate("-3*2"), Ok(-6.0));
    }

    #[test]
    fn formats_integral_values_as_integers() {
        assert_eq!(format_number(50.0), "50");
        assert_eq!(format_number(-6.0), "-6");
    }

    #[test]
    fn formats_fractional_values_as_reals() {
        assert_eq!(format_number(3.5), "3.5");
    }
}
