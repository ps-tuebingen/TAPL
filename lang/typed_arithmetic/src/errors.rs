use crate::{eval::Value, parser::Rule, syntax::Type};
use pest::error::Error as PestErr;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    TypeMismatch { found: Type, expected: Type },
    BadValue { found: Value, expected: String },
    Pest(Box<PestErr<Rule>>),
    MissingInput(String),
    RemainingInput(Rule),
    UnexpectedRule { found: Rule, expected: String },
    UnknownKeyword(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::TypeMismatch { found, expected } => {
                write!(f, "Type Mismatch, found {found}, expected {expected}")
            }
            Error::BadValue { found, expected } => {
                write!(f, "Bad Value {found}, expected {expected}")
            }
            Error::Pest(err) => write!(f, "Error in Pest: {err}"),
            Error::MissingInput(missing) => write!(f, "Missing Input {missing}"),
            Error::RemainingInput(r) => write!(f, "Remaining Input {r:?}"),
            Error::UnexpectedRule { found, expected } => {
                write!(f, "Unexpected Rule {found:?}, expected {expected}")
            }
            Error::UnknownKeyword(kw) => write!(f, "Unknown Keyword {kw}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<PestErr<Rule>> for Error {
    fn from(err: PestErr<Rule>) -> Error {
        Error::Pest(Box::new(err))
    }
}
