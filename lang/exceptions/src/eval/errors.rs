use super::values::Value;
use crate::syntax::{Term, Var};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    FreeVar(Var),
    NotAFunction(Value),
    NotANumber(Value),
    NotABool(Value),
    NotAValue(Term),
    Stuck(Term),
    ExceptionVal(Value),
    Exception,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FreeVar(v) => write!(f, "Variable {v} appears free"),
            Error::NotAFunction(v) => write!(f, "{v} should be a function value"),
            Error::NotANumber(v) => write!(f, "{v} should be a number"),
            Error::NotAValue(t) => write!(f, "Term {t} is not a value"),
            Error::NotABool(v) => write!(f, "{v} should be a boolean value"),
            Error::Stuck(t) => write!(f, "Term {t} is stuck"),
            Error::ExceptionVal(v) => write!(f, "Evaluation encountered an exception: {v}"),
            Error::Exception => write!(f, "Evaluation encountered and exception"),
        }
    }
}

impl std::error::Error for Error {}
