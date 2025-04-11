use crate::{Label, Type, TypeVar, Value, Var};
use pest::{error::Error as PestErr, RuleType};
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    FreeVariable(Var),
    FreeTypeVariable(TypeVar),
    UndefinedLabel(Label),
    UndefinedName(String),
    DefinedMultipleTimes(String),
    Subtype { sub: Type, sup: Type },
    NameMismatch { found: String, expected: String },
    TypeMismatch { found: Type, expected: String },
    ValueMismatch { found: Value, expected: String },
    Arity { found: usize, expected: usize },
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
            ErrorKind::UndefinedName(name) => write!(f, "Name {name} was not defined"),
            ErrorKind::DefinedMultipleTimes(name) => write!(f, "{name} was defined multiple times"),
            ErrorKind::Subtype { sub, sup } => write!(f, "{sub} is not a subtype of {sup}"),
            ErrorKind::NameMismatch { found, expected } => {
                write!(f, "Names {found} and {expected} should be equal")
            }
            ErrorKind::Arity { found, expected } => {
                write!(f, "Arity mismatch, expected {expected}, found {found}")
            }
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
