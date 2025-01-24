use crate::{
    eval::Value,
    syntax::{
        kinds::Kind,
        terms::{Term, Var},
        types::{Type, TypeVar},
    },
    Label,
};
use std::fmt;

#[derive(Debug)]
pub enum ErrorKind {
    FreeVar(Var),
    FreeTypeVar(TypeVar),
    UndefinedLabel(Label),
    NoSubtype { lower: Type, upper: Type },
    BadValue { found: Value, expected: String },
    BadKind { found: Kind, expected: String },
    BadType { found: Type, expected: String },
}

#[derive(Debug)]
pub enum ErrorLocation {
    Typechecking,
    Evaluation,
    TypeEquivalence,
    KindChecking,
    SubtypeChecking,
    TypeReduction,
}

#[derive(Debug)]
pub enum ErrorContext {
    Term(Term),
    Types(Type, Type),
    Type(Type),
}

#[derive(Debug)]
pub struct Error {
    pub loc: ErrorLocation,
    pub context: ErrorContext,
    pub kind: ErrorKind,
}

impl Error {
    pub fn equiv<T: Into<Type>, U: Into<Type>>(kind: ErrorKind, left: T, right: U) -> Error {
        Error {
            loc: ErrorLocation::TypeEquivalence,
            context: ErrorContext::Types(left.into(), right.into()),
            kind,
        }
    }

    pub fn check<T: Into<Term> + Clone>(kind: ErrorKind, term: &T) -> Error {
        Error {
            loc: ErrorLocation::Typechecking,
            context: ErrorContext::Term(term.clone().into()),
            kind,
        }
    }

    pub fn kind<T: Into<Type> + Clone>(kind: ErrorKind, ty: &T) -> Error {
        Error {
            loc: ErrorLocation::KindChecking,
            context: ErrorContext::Type(ty.clone().into()),
            kind,
        }
    }

    pub fn sub_ty<T: Into<Type> + Clone, U: Into<Type> + Clone>(
        kind: ErrorKind,
        lower: &T,
        upper: &U,
    ) -> Error {
        Error {
            loc: ErrorLocation::SubtypeChecking,
            context: ErrorContext::Types(lower.clone().into(), upper.clone().into()),
            kind,
        }
    }

    pub fn eval<T: Into<Term>>(kind: ErrorKind, t: T) -> Error {
        Error {
            loc: ErrorLocation::Evaluation,
            context: ErrorContext::Term(t.into()),
            kind,
        }
    }

    pub fn ty_red<T: Into<Type>>(kind: ErrorKind, ty: T) -> Error {
        Error {
            loc: ErrorLocation::TypeReduction,
            context: ErrorContext::Type(ty.into()),
            kind,
        }
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::FreeVar(v) => write!(f, "Variable {v} appears free"),
            ErrorKind::FreeTypeVar(v) => write!(f, "Type Variable {v} appears free"),
            ErrorKind::UndefinedLabel(label) => write!(f, "Undefined Label {label}"),
            ErrorKind::NoSubtype { lower, upper } => write!(f, "{lower} is no subtype of {upper}"),
            ErrorKind::BadValue { found, expected } => {
                write!(f, "Unexpectd value {found}, expected {expected}")
            }
            ErrorKind::BadKind { found, expected } => {
                write!(f, "Unexpeced kind {found}, expected {expected}")
            }
            ErrorKind::BadType { found, expected } => {
                write!(f, "Unexpected type {found}, expected {expected}")
            }
        }
    }
}

impl fmt::Display for ErrorLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorLocation::Evaluation => f.write_str("Evaluation"),
            ErrorLocation::Typechecking => f.write_str("Type Checking"),
            ErrorLocation::TypeEquivalence => f.write_str("Type Equivalence Checking"),
            ErrorLocation::KindChecking => f.write_str("Kind Checking"),
            ErrorLocation::SubtypeChecking => f.write_str("Subtype Checking"),
            ErrorLocation::TypeReduction => f.write_str("Type Reduction"),
        }
    }
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorContext::Term(t) => t.fmt(f),
            ErrorContext::Types(left, right) => write!(f, "{left}, {right}"),
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
