use super::terms::Term;
use common::errors::ErrorKind;
use eval::values::{Lambda, Value as ValueTrait, ValueGroup};
use syntax::untyped::Untyped;

use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(Lambda<Term, Untyped>),
}

impl eval::values::Value for Value {
    type Term = Term;
}

impl ValueGroup for Value {
    type Term = Term;
    type Type = Untyped;

    fn into_lambda(self) -> Result<Lambda<Term, Untyped>, ErrorKind> {
        match self {
            Value::Lambda(lam) => Ok(lam),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Lambda(lam) => lam.fmt(f),
        }
    }
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda(lam) => lam.into_term().into(),
        }
    }
}

impl From<Lambda<Term, Untyped>> for Value {
    fn from(lam: Lambda<Term, Untyped>) -> Value {
        Value::Lambda(lam)
    }
}
