use crate::{syntax::Var, types::Type};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    FreeVar(Var),
    NotAFunctionType(Type),
    TypeMismatch(Type, Type),
    NotUniversal(Type),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FreeVar(v) => write!(f, "Variable {v} cannot appear free"),
            Error::NotAFunctionType(ty) => write!(f, "{ty} should be a function type"),
            Error::TypeMismatch(ty1, ty2) => write!(f, "{ty1} and {ty2} should be equal"),
            Error::NotUniversal(ty) => write!(f, "{ty} is not a generalized type"),
        }
    }
}

impl std::error::Error for Error {}
