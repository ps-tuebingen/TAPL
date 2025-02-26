use crate::{syntax::Term, types::Type, Var};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    BadTarget { t: Term, ty: Type },
    TypeMismatch { ty1: Type, ty2: Type },
    CannotInfer { t: Term },
    UnexpectedType { found: Type, expected: Type },
    FreeVariable { var: Var },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::BadTarget { t, ty } => write!(f, "Cannot type {t} with type {ty}."),
            Error::TypeMismatch { ty1, ty2 } => write!(f, "Types {ty1} and {ty2} are not equal."),
            Error::CannotInfer { t } => write!(f, "Cannot infer type of term {t}"),
            Error::UnexpectedType { found, expected } => {
                write!(f, "Unexpected type {found}, expected: {expected}")
            }
            Error::FreeVariable { var } => write!(f, "Variable {var} was used but never bound."),
        }
    }
}

impl std::error::Error for Error {}
