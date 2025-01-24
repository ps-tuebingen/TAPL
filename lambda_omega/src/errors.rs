use crate::{
    eval::Value,
    kinds::Kind,
    syntax::Var,
    types::{Type, TypeVar},
};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    FreeVar(Var),
    FreeTypeVar(TypeVar),
    KindMismatch { found: Kind, expected: String },
    TypeMismatch { found: Type, expected: String },
    BadValue { found: Value, expected: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FreeVar(v) => write!(f, "Variable {v} appears free"),
            Error::BadValue { found, expected } => {
                write!(f, "Unexpected Value {found}, expected {expected}")
            }
            Error::FreeTypeVar(var) => write!(f, "Type variable {var} appears free"),
            Error::KindMismatch { found, expected } => {
                write!(f, "Unexpected kind {found}, expected {expected}")
            }
            Error::TypeMismatch { found, expected } => {
                write!(f, "Unexpected type {found}, expected {expected}")
            }
        }
    }
}

impl std::error::Error for Error {}
