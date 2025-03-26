use crate::{
    eval::Value,
    syntax::{Label, Term, Var},
    types::{Type, TypeVar},
};
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    BadValue { found: Value, expected: String },
    TypeMismatch { found: Type, expected: String },
    FreeTypeVar(TypeVar),
    FreeVar(Var),
    UndefinedLabel(Label),
}

#[derive(Debug)]
pub enum ErrorLocation {
    Typecheck,
    Evaluation,
}

#[derive(Debug)]
pub enum ErrorContext {
    Term(Term),
    Subtype(Type, Type),
}

impl From<Term> for ErrorContext {
    fn from(t: Term) -> ErrorContext {
        ErrorContext::Term(t)
    }
}

impl From<(Type, Type)> for ErrorContext {
    fn from((lower, upper): (Type, Type)) -> ErrorContext {
        ErrorContext::Subtype(lower, upper)
    }
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub loc: ErrorLocation,
    pub context: ErrorContext,
}

impl Error {
    pub fn check<T: Into<Term> + Clone>(kind: ErrorKind, t: &T) -> Error {
        Error {
            kind,
            loc: ErrorLocation::Typecheck,
            context: ErrorContext::Term(t.clone().into()),
        }
    }

    pub fn sub_ty(kind: ErrorKind, lower: Type, upper: Type) -> Error {
        Error {
            kind,
            loc: ErrorLocation::Typecheck,
            context: ErrorContext::Subtype(lower, upper),
        }
    }

    pub fn eval<T: Into<Term> + Clone>(kind: ErrorKind, term: &T) -> Error {
        Error {
            kind,
            loc: ErrorLocation::Evaluation,
            context: ErrorContext::Term(term.clone().into()),
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::BadValue { found, expected } => {
                write!(f, "Unexpected value {found}, expected {expected}")
            }
            ErrorKind::FreeVar(v) => write!(f, "Variable {v} appears free"),
            ErrorKind::TypeMismatch { found, expected } => {
                write!(f, "Unexpected type {found}, expected {expected}")
            }
            ErrorKind::FreeTypeVar(var) => write!(f, "Type Variable {var} appars free"),
            ErrorKind::UndefinedLabel(label) => write!(f, "Undefined Label {label}"),
        }
    }
}

impl fmt::Display for ErrorLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorLocation::Typecheck => f.write_str("Typecheck"),
            ErrorLocation::Evaluation => f.write_str("Evaluation"),
        }
    }
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorContext::Term(t) => t.fmt(f),
            ErrorContext::Subtype(lower, upper) => write!(f, "{lower}<:{upper}"),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error during {} for {}: {}",
            self.loc, self.context, self.kind
        )
    }
}

impl std::error::Error for Error {}
