//! A simple arithmetic calculator.

/// A binary arithmetic operator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    Add,
    Subtract,
}

impl Operator {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Self::Add),
            '-' => Some(Self::Subtract),
            _ => None,
        }
    }

    fn apply(self, left: f64, right: f64) -> f64 {
        match self {
            Self::Add => left + right,
            Self::Subtract => left - right,
        }
    }
}

/// Evaluate a binary arithmetic expression such as `"3+2"` or `"10-7"`.
///
/// Whitespace is ignored. Operands may be real numbers and may carry a sign,
/// so `"-3+2"` and `"3--2"` are both valid.
pub fn evaluate(input: &str) -> Result<f64, String> {
    let expression: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    // Skip the first character so that a leading sign belongs to the left operand.
    let (index, operator) = expression
        .char_indices()
        .skip(1)
        .find_map(|(i, c)| Operator::from_char(c).map(|op| (i, op)))
        .ok_or_else(|| format!("no operator found in \"{input}\""))?;
    let left = parse_operand(&expression[..index])?;
    let right = parse_operand(&expression[index + 1..])?;
    Ok(operator.apply(left, right))
}

fn parse_operand(operand: &str) -> Result<f64, String> {
    operand
        .parse()
        .map_err(|_| format!("invalid number \"{operand}\""))
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
