use super::errors::Error;
use crate::{
    terms::{Loc, Term, Var},
    types::Type,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda { var: Var, annot: Type, body: Term },
    Unit,
    Loc(Loc),
}

impl Value {
    pub fn from_term(t: Term) -> Result<Value, Error> {
        t.try_into()
    }
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda { var, annot, body } => Term::Lambda {
                var,
                annot,
                body: Box::new(body),
            },
            Value::Unit => Term::Unit,
            Value::Loc(loc) => Term::Loc(loc),
        }
    }
}

impl TryFrom<Term> for Value {
    type Error = Error;
    fn try_from(t: Term) -> Result<Value, Error> {
        match t {
            Term::Lambda { var, annot, body } => Ok(Value::Lambda {
                var,
                annot,
                body: *body,
            }),
            Term::Unit => Ok(Value::Unit),
            Term::Loc(loc) => Ok(Value::Loc(loc)),
            _ => Err(Error::NotAValue(t)),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Value as Into<Term>>::into(self.clone()).fmt(f)
    }
}
