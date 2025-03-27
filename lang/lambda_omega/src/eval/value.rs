use crate::{
    errors::Error,
    kinds::Kind,
    syntax::{Term, Var},
    types::{Type, TypeVar},
};
use std::fmt;

#[derive(Clone, Debug)]
pub enum Value {
    Unit,
    Lambda {
        var: Var,
        annot: Type,
        body: Term,
    },
    TyLambda {
        var: TypeVar,
        annot: Kind,
        body: Term,
    },
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

    pub fn as_tylambda(self) -> Result<(TypeVar, Kind, Term), Error> {
        if let Value::TyLambda { var, annot, body } = self {
            Ok((var, annot, body))
        } else {
            Err(Error::BadValue {
                found: self,
                expected: "Type Abstraction".to_owned(),
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
            Value::TyLambda { var, annot, body } => Term::TyLambda {
                var,
                kind: annot,
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
