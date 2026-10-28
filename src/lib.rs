//! A simple calculator that evaluates binary arithmetic expressions such as `3+2`, `10-7`, `3*2`, or `100/2`.

use std::fmt;

/// An error encountered while evaluating an expression.
#[derive(Debug, PartialEq)]
pub enum CalcError {
    /// The input could not be parsed as `<number><operator><number>`.
    Parse(String),
    /// The expression attempted to divide by zero.
    DivideByZero,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::Parse(input) => write!(f, "cannot parse expression: {input:?}"),
            CalcError::DivideByZero => write!(f, "cannot divide by zero"),
        }
    }
}

impl std::error::Error for CalcError {}

/// A binary arithmetic operator.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Operator::Add),
            '-' => Some(Operator::Subtract),
            '*' => Some(Operator::Multiply),
            '/' => Some(Operator::Divide),
            _ => None,
        }
    }

    fn apply(self, left: f64, right: f64) -> Result<f64, CalcError> {
        match self {
            Operator::Add => Ok(left + right),
            Operator::Subtract => Ok(left - right),
            Operator::Multiply => Ok(left * right),
            Operator::Divide if right == 0.0 => Err(CalcError::DivideByZero),
            Operator::Divide => Ok(left / right),
        }
    }
}

/// Evaluate a binary arithmetic expression of the form `<number><operator><number>`.
///
/// The operator is the first operator character that follows a non-empty left operand, so a
/// leading sign on the left operand is never mistaken for an operator.
pub fn evaluate(input: &str) -> Result<f64, CalcError> {
    let parse_error = || CalcError::Parse(input.to_string());
    let (index, operator) = input
        .char_indices()
        .filter(|&(i, _)| !input[..i].trim().is_empty())
        .find_map(|(i, c)| Operator::from_char(c).map(|op| (i, op)))
        .ok_or_else(parse_error)?;
    let operand = |s: &str| s.trim().parse::<f64>().map_err(|_| parse_error());
    // Every operator is a single ASCII byte.
    let left = operand(&input[..index])?;
    let right = operand(&input[index + 1..])?;
    operator.apply(left, right)
}

/// Format a number, printing integral values without a fractional part.
pub fn format_number(x: f64) -> String {
    // `f64`'s `Display` already omits the fractional part of integral values.
    // Adding 0.0 normalizes -0.0 to 0.0.
    (x + 0.0).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multiplies() {
        assert_eq!(evaluate("3*2"), Ok(6.0));
    }

    #[test]
    fn divides() {
        assert_eq!(evaluate("100/2"), Ok(50.0));
        assert_eq!(evaluate("7/2"), Ok(3.5));
    }

    #[test]
    fn ignores_surrounding_whitespace() {
        assert_eq!(evaluate(" 3 * 2 "), Ok(6.0));
    }

    #[test]
    fn handles_real_operands() {
        assert_eq!(evaluate("1.5*2"), Ok(3.0));
    }

    #[test]
    fn handles_negative_operands() {
        assert_eq!(evaluate("-3*2"), Ok(-6.0));
        assert_eq!(evaluate("6/-2"), Ok(-3.0));
    }

    #[test]
    fn rejects_division_by_zero() {
        assert_eq!(evaluate("1/0"), Err(CalcError::DivideByZero));
        assert_eq!(evaluate("0/0"), Err(CalcError::DivideByZero));
    }

    #[test]
    fn rejects_malformed_input() {
        assert!(matches!(evaluate("3*"), Err(CalcError::Parse(_))));
        assert!(matches!(evaluate("abc"), Err(CalcError::Parse(_))));
        assert!(matches!(evaluate(""), Err(CalcError::Parse(_))));
    }

    #[test]
    fn adds() {
        assert_eq!(evaluate("3+2"), Ok(5.0));
    }

    #[test]
    fn subtracts() {
        assert_eq!(evaluate("10-7"), Ok(3.0));
    }

    #[test]
    fn subtraction_can_be_negative() {
        assert_eq!(evaluate("2-5"), Ok(-3.0));
    }

    #[test]
    fn ignores_whitespace() {
        assert_eq!(evaluate(" 3 + 2 "), Ok(5.0));
    }

    #[test]
    fn accepts_real_operands() {
        assert_eq!(evaluate("1.25+2.5"), Ok(3.75));
    }

    #[test]
    fn accepts_negative_operands() {
        assert_eq!(evaluate("-3+2"), Ok(-1.0));
        assert_eq!(evaluate("3--2"), Ok(5.0));
        assert_eq!(evaluate("3+-2"), Ok(1.0));
    }

    #[test]
    fn rejects_missing_operand() {
        assert!(evaluate("3+").is_err());
        assert!(evaluate("+3").is_err());
    }

    #[test]
    fn rejects_missing_operator() {
        assert!(evaluate("3").is_err());
        assert!(evaluate("").is_err());
    }

    #[test]
    fn rejects_non_numeric_operands() {
        assert!(evaluate("a+2").is_err());
        assert!(evaluate("3+b").is_err());
    }

    #[test]
    fn formats_integers_without_fraction() {
        assert_eq!(format_number(5.0), "5");
        assert_eq!(format_number(-3.0), "-3");
        assert_eq!(format_number(0.0), "0");
        assert_eq!(format_number(-0.0), "0");
    }

    #[test]
    fn formats_reals_with_fraction() {
        assert_eq!(format_number(3.5), "3.5");
        assert_eq!(format_number(-0.25), "-0.25");
    }
}
