use super::Error;
use crate::{
    syntax::{Lambda, Term, Unit, Var},
    types::Type,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda { var: Var, annot: Type, body: Term },
    Const(i64),
    Unit,
    True,
    False,
}

impl Value {
    pub fn into_lambda(self) -> Result<(Var, Type, Term), Error> {
        if let Value::Lambda { var, annot, body } = self {
            Ok((var, annot, body))
        } else {
            Err(Error::NotAFunction(self))
        }
    }

    pub fn into_num(self) -> Result<i64, Error> {
        if let Value::Const(i) = self {
            Ok(i)
        } else {
            Err(Error::NotANumber(self))
        }
    }
}

impl From<Lambda> for Value {
    fn from(lam: Lambda) -> Value {
        Value::Lambda {
            var: lam.var,
            annot: lam.annot,
            body: *lam.body,
        }
    }
}

impl From<Unit> for Value {
    fn from(_: Unit) -> Value {
        Value::Unit
    }
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda { var, annot, body } => Lambda {
                var,
                annot,
                body: Box::new(body),
            }
            .into(),
            Value::Unit => Unit.into(),
            Value::True => Term::True,
            Value::False => Term::False,
            Value::Const(i) => Term::Const(i),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Value as Into<Term>>::into(self.clone()).fmt(f)
    }
}
