use crate::terms::Term;
use common::{
    language::LanguageValue,
    values::{False, Lambda, Num, True, TyLambda, Unit, Value as ValueTrait},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Unit(Unit<Term>),
    True(True<Term>),
    False(False<Term>),
    Num(Num<Term>),
    Lambda(Lambda<Term>),
    TyLambda(TyLambda<Term>),
}

impl common::values::Value for Value {
    type Term = Term;
}

impl LanguageValue for Value {
    type Term = Term;
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda(lam) => lam.into_term().into(),
            Value::TyLambda(tylam) => tylam.into_term().into(),
            Value::Unit(u) => u.into_term().into(),
            Value::True(tru) => tru.into_term().into(),
            Value::False(fls) => fls.into_term().into(),
            Value::Num(num) => num.into_term().into(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Lambda(lam) => lam.fmt(f),
            Value::TyLambda(tylam) => tylam.fmt(f),
            Value::Unit(u) => u.fmt(f),
            Value::True(tru) => tru.fmt(f),
            Value::False(fls) => fls.fmt(f),
            Value::Num(num) => num.fmt(f),
        }
    }
}

impl From<Unit<Term>> for Value {
    fn from(u: Unit<Term>) -> Value {
        Value::Unit(u)
    }
}
impl From<True<Term>> for Value {
    fn from(tru: True<Term>) -> Value {
        Value::True(tru)
    }
}
impl From<False<Term>> for Value {
    fn from(fls: False<Term>) -> Value {
        Value::False(fls)
    }
}
impl From<Num<Term>> for Value {
    fn from(num: Num<Term>) -> Value {
        Value::Num(num)
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
