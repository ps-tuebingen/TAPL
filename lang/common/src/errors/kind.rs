use crate::{Label, Type, TypeVar, Value, Var};
use pest::{error::Error as PestErr, RuleType};
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    FreeVariable(Var),
    FreeTypeVariable(TypeVar),
    UndefinedLabel(Label),
    TypeMismatch { found: Type, expected: String },
    ValueMismatch { found: Value, expected: String },
    // Parsing Errors
    Pest(String),
    MissingInput(String),
    RemainingInput(String),
    UnknownKeyword(String),
    UnexpectedRule { found: String, expected: String },
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::FreeVariable(v) => write!(f, "Variable {v} appears free"),
            ErrorKind::FreeTypeVariable(v) => write!(f, "Type Variable {v} appears free"),
            ErrorKind::UndefinedLabel(lb) => write!(f, "Label {lb} was not defined"),
            ErrorKind::TypeMismatch { found, expected } => {
                write!(f, "Unexpected type {found}, expected {expected}")
            }
            ErrorKind::ValueMismatch { found, expected } => {
                write!(f, "Unexpected value {found}, expected {expected}")
            }
            ErrorKind::Pest(msg) => write!(f, "Error in Pest: {msg}"),
            ErrorKind::MissingInput(missing) => write!(f, "Missing Input {missing}"),
            ErrorKind::RemainingInput(remaining) => write!(f, "Remaining Input {remaining}"),
            ErrorKind::UnknownKeyword(kw) => write!(f, "Unknown Keyword {kw}"),
            ErrorKind::UnexpectedRule { found, expected } => {
                write!(f, "Unexpeced rule {found}, expected {expected}")
            }
        }
    }
}

impl<T: RuleType> From<PestErr<T>> for ErrorKind {
    fn from(err: PestErr<T>) -> ErrorKind {
        ErrorKind::Pest(err.to_string())
    }
}
