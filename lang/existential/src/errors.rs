use crate::{
    eval::Value,
    terms::{Term, Var},
    types::Type,
    Label,
};
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    FreeVar(Var),
    UndefinedLabel(Label),
    TypeMismatch { found: Type, expected: String },
    BadValue { found: Value, expected: String },
}

impl ErrorKind {
    pub fn ty_mismatch(found: &Type, expected: &str) -> ErrorKind {
        ErrorKind::TypeMismatch {
            found: found.clone(),
            expected: expected.to_owned(),
        }
    }

    pub fn value(found: &Value, expected: &str) -> ErrorKind {
        ErrorKind::BadValue {
            found: found.clone(),
            expected: expected.to_owned(),
        }
    }

    pub fn label(label: &str) -> ErrorKind {
        ErrorKind::UndefinedLabel(label.to_owned())
    }

    pub fn var(var: &str) -> ErrorKind {
        ErrorKind::FreeVar(var.to_owned())
    }
}

#[derive(Debug)]
pub enum ErrorLocation {
    Typecheck,
    Eval,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub loc: ErrorLocation,
    pub term: Term,
}

impl Error {
    pub fn check<T: Into<Term> + Clone>(kind: ErrorKind, t: &T) -> Error {
        Error {
            kind,
            term: t.clone().into(),
            loc: ErrorLocation::Typecheck,
        }
    }

    pub fn eval<T: Into<Term> + Clone>(kind: ErrorKind, t: &T) -> Error {
        Error {
            kind,
            term: t.clone().into(),
            loc: ErrorLocation::Typecheck,
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::FreeVar(v) => write!(f, "Variable {v} appears free"),
            ErrorKind::UndefinedLabel(label) => write!(f, "Label {label} was used but not defined"),
            ErrorKind::TypeMismatch { found, expected } => {
                write!(f, "Unexpected type {found}, expected {expected}")
            }
            ErrorKind::BadValue { found, expected } => {
                write!(f, "Unexpected value {found}, expected {expected}")
            }
        }
    }
}

impl fmt::Display for ErrorLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorLocation::Eval => f.write_str("Evaluation"),
            ErrorLocation::Typecheck => f.write_str("Type checking"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error during {} of {}: {}",
            self.loc, self.term, self.kind
        )
    }
}

impl std::error::Error for Error {}
