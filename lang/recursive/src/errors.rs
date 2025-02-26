use super::{terms::Term, terms::Var, types::Type, Label};
use std::fmt;

#[derive(Debug)]
pub enum ErrorLocation {
    TypeChecking,
    Evaluation,
}

#[derive(Debug)]
pub enum ErrorKind {
    FreeVar(Var),
    UndefinedLabel(Label),
    EmptyCase,
    UnexpectedType { found: Type, expected: String },
    UnexpectedTerm { found: Term, expected: String },
}

impl ErrorKind {
    pub fn unexpected_type(found: &Type, expected: &str) -> ErrorKind {
        ErrorKind::UnexpectedType {
            found: found.clone(),
            expected: expected.to_owned(),
        }
    }

    pub fn unexpected_term(found: &Term, expected: &str) -> ErrorKind {
        ErrorKind::UnexpectedTerm {
            found: found.clone(),
            expected: expected.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    during: ErrorLocation,
    with: Term,
}

impl Error {
    pub fn check<T: Into<Term> + Clone>(kind: ErrorKind, t: &T) -> Error {
        Error {
            kind,
            during: ErrorLocation::TypeChecking,
            with: t.clone().into(),
        }
    }

    pub fn eval<T: Into<Term> + Clone>(kind: ErrorKind, t: &T) -> Error {
        Error {
            kind,
            during: ErrorLocation::Evaluation,
            with: t.clone().into(),
        }
    }
}

impl fmt::Display for ErrorLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorLocation::TypeChecking => f.write_str("Type Checking"),
            ErrorLocation::Evaluation => f.write_str("Evaluation"),
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::FreeVar(var) => write!(f, "Free Variable {var}"),
            ErrorKind::UndefinedLabel(label) => write!(f, "Undefined Label {label}"),
            ErrorKind::UnexpectedType { found, expected } => {
                write!(f, "Unexpected Type: found {found}, expected {expected}")
            }
            ErrorKind::UnexpectedTerm { found, expected } => {
                write!(f, "Unexpected term: found {found}, expected {expected}")
            }
            ErrorKind::EmptyCase => f.write_str("No patterns in case expression"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error: {}, during {} {}",
            self.kind, self.during, self.with
        )
    }
}

impl std::error::Error for Error {}
