use super::values::Value;
use crate::terms::{Loc, Term, Var};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    FreeVar(Var),
    LocationNotFound(Loc),
    NotAFunction(Value),
    NotALocation(Value),
    NotANumber(Value),
    NotAValue(Term),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::FreeVar(v) => write!(f, "Variable {v} appears free"),
            Error::LocationNotFound(loc) => write!(f, "Could not find location {loc} in store"),
            Error::NotAFunction(v) => write!(f, "{v} should be a function value"),
            Error::NotALocation(val) => write!(f, "{val} should be a store location"),
            Error::NotANumber(val) => write!(f, "{val} should be a number"),
            Error::NotAValue(t) => write!(f, "Term {t} is not a value"),
        }
    }
}

impl std::error::Error for Error {}
