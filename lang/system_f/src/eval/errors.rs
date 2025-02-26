use super::Value;
use crate::syntax::Var;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    FreeVar(Var),
    NotAFunction(Value),
    NotATyAbs(Value),
    NotAList(Value),
    NotABool(Value),
    EmptyList(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FreeVar(v) => write!(f, "Variable {v} cannot appear free"),
            Error::NotAFunction(val) => write!(f, "{val} is not a function"),
            Error::NotATyAbs(val) => write!(f, "{val} is not a type abstraction"),
            Error::NotAList(val) => write!(f, "{val} is not a list value"),
            Error::NotABool(val) => write!(f, "{val} is not a boolean value"),
            Error::EmptyList(fun) => write!(f, "Cannot apply {fun} to empty list"),
        }
    }
}

impl std::error::Error for Error {}
