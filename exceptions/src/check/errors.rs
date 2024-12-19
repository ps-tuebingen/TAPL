use crate::{syntax::Var, types::Type};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    FreeVar(Var),
    NotAFunctionType(Type),
    TypeMismatch { found: Type, expected: Type },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FreeVar(v) => write!(f, "Cannot type free variable {v}"),
            Error::NotAFunctionType(ty) => write!(f, "{ty} is not a function type"),
            Error::TypeMismatch { found, expected } => {
                write!(f, "Types do not match, found:{found}, expected:{expected}")
            }
        }
    }
}
