use super::terms::Term;
use common::{
    errors::ErrorKind,
    language::LanguageValue,
    values::{Lambda, TyLambda, Value as ValueTrait},
};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Lambda(Lambda<Term>),
    TyLambda(TyLambda<Term>),
}

impl common::values::Value for Value {
    type Term = Term;
}

impl LanguageValue for Value {
    type Term = Term;

    fn into_lambda(self) -> Result<Lambda<Term>, ErrorKind> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Lambda Abstraction".to_owned(),
            })
        }
    }

    fn into_tylambda(self) -> Result<TyLambda<Term>, ErrorKind> {
        if let Value::TyLambda(lam) = self {
            Ok(lam)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Operator Abstraction".to_owned(),
            })
        }
    }
}

impl From<Value> for Term {
    fn from(v: Value) -> Term {
        match v {
            Value::Lambda(lam) => lam.into_term().into(),
            Value::TyLambda(tylam) => tylam.into_term().into(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Lambda(lam) => lam.fmt(f),
            Value::TyLambda(tylam) => tylam.fmt(f),
        }
    }
}

impl From<Lambda<Term>> for Value {
    fn from(lam: Lambda<Term>) -> Value {
        Value::Lambda(lam)
    }
}

impl From<TyLambda<Term>> for Value {
    fn from(tylam: TyLambda<Term>) -> Value {
        Value::TyLambda(tylam)
    }
}
