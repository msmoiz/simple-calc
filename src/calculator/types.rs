use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Operand(isize),
    Operator(Operator),
    LeftParen,
    RightParen,
}

/// Defines the standard result type used across the Calculator interface.
pub type Result<T> = std::result::Result<T, Error>;

/// Defines the standard error type used across the Calculator interface.
#[derive(Debug, Clone)]
pub enum Error {
    InvalidCharacter(char, usize),
    ZeroLengthExpression,
    InvalidExpression(String),
    MismatchedParentheses,
    DivideByZero(isize, isize),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidCharacter(char, position) => write!(
                f,
                "encountered invalid character {} in expression at position {}",
                char, position
            ),
            Error::ZeroLengthExpression => write!(
                f,
                "input expression appears to have zero length and cannot be evaluated"
            ),
            Error::InvalidExpression(detail) => {
                write!(f, "input expression is invalid: {}", detail)
            }
            Error::MismatchedParentheses => {
                write!(f, "input expression contains mismatched parentheses")
            }
            Error::DivideByZero(a, b) => write!(f, "attempted to divide by zero: {} / {}", a, b),
        }
    }
}
