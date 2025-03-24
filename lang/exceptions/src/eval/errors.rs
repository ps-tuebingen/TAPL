use super::values::Value;
use crate::syntax::{Term, Var};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    FreeVar(Var),
    NotAFunction(Value),
    NotAValue(Term),
    Stuck(Term),
    ExceptionVal(Value),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FreeVar(v) => write!(f, "Variable {v} appears free"),
            Error::NotAFunction(v) => write!(f, "{v} should be a function value"),
            Error::NotAValue(t) => write!(f, "Term {t} is not a value"),
            Error::Stuck(t) => write!(f, "Term {t} is stuck"),
            Error::ExceptionVal(v) => write!(f, "Evaluation encountered an exception: {v}"),
        }
    }
}

impl std::error::Error for Error {}
