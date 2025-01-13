use crate::{
    syntax::{Lambda, Term, TyLambda, Var},
    types::{TyVar, Type},
};
use std::fmt;

#[derive(Debug)]
pub enum Value {
    Lambda { var: Var, annot: Type, body: Term },
    TyLambda { var: TyVar, body: Term },
}

impl From<Value> for Term {
    fn from(v: Value) -> Term {
        match v {
            Value::Lambda { var, annot, body } => Lambda::new(&var, annot, body).into(),
            Value::TyLambda { var, body } => TyLambda::new(&var, body).into(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Lambda { var, annot, body } => write!(f, "λ{}:{}.{}", var, annot, body),
            Value::TyLambda { var, body } => write!(f, "λ{}.{}", var, body),
        }
    }
}
