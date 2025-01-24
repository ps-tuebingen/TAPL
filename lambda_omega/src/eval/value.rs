use crate::{
    errors::Error,
    syntax::{Term, Var},
    types::Type,
};
use std::fmt;

#[derive(Clone, Debug)]
pub enum Value {
    Unit,
    Lambda { var: Var, annot: Type, body: Term },
}

impl Value {
    pub fn as_lambda(self) -> Result<(Var, Type, Term), Error> {
        if let Value::Lambda { var, annot, body } = self {
            Ok((var, annot, body))
        } else {
            Err(Error::BadValue {
                found: self,
                expected: "Lambda Abstraction".to_owned(),
            })
        }
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
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Value as Into<Term>>::into(self.clone()).fmt(f)
    }
}
