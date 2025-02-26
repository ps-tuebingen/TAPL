use crate::{
    eval::Value,
    syntax::{
        kinds::Kind,
        terms::{Term, Var},
        types::{Type, TypeVar},
        Label,
    },
};
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    FreeVar(Var),
    FreeTypeVar(TypeVar),
    UndefinedLabel(Label),
    BadValue { found: Value, expected: String },
    KindMismatch { found: Kind, expected: String },
    TypeMismatch { found: Type, expected: String },
}

#[derive(Debug)]
pub enum ErrorLocation {
    Typecheck,
    KindCheck,
    Eval,
}

#[derive(Debug)]
pub enum ErrorContext {
    Term(Term),
    Type(Type),
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
    pub fn eval<T: Into<Term> + Clone>(kind: ErrorKind, t: &T) -> Error {
        Error {
            kind,
            loc: ErrorLocation::Eval,
            context: ErrorContext::Term(t.clone().into()),
        }
    }

    pub fn kinding<T: Into<Type> + Clone>(kind: ErrorKind, t: &T) -> Error {
        Error {
            kind,
            loc: ErrorLocation::KindCheck,
            context: ErrorContext::Type(t.clone().into()),
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::FreeVar(v) => write!(f, "Variable {v} appears free"),
            ErrorKind::FreeTypeVar(v) => write!(f, "Type variable {v} appears free"),
            ErrorKind::UndefinedLabel(label) => write!(f, "Label {label} was used but not defined"),
            ErrorKind::BadValue { found, expected } => {
                write!(f, "Unexpected value {found}, expected {expected}")
            }
            ErrorKind::KindMismatch { found, expected } => {
                write!(f, "Unexpected kind {found}, expected {expected}")
            }
            ErrorKind::TypeMismatch { found, expected } => {
                write!(f, "Unexpected type {found}, expected {expected}")
            }
        }
    }
}

impl fmt::Display for ErrorLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorLocation::Typecheck => f.write_str("Type checking"),
            ErrorLocation::Eval => f.write_str("Evaluation"),
            ErrorLocation::KindCheck => f.write_str("Kind checking"),
        }
    }
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorContext::Term(t) => t.fmt(f),
            ErrorContext::Type(ty) => ty.fmt(f),
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
