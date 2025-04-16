use crate::terms::Term;
use common::{
    language::LanguageValue,
    values::{False, Num, True, Value as ValueTrait},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    True(True<Term>),
    False(False<Term>),
    Num(Num<Term>),
}

impl common::values::Value for Value {
    type Term = Term;
}

impl LanguageValue for Value {
    type Term = Term;
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::True(tru) => tru.fmt(f),
            Value::False(fls) => fls.fmt(f),
            Value::Num(num) => num.fmt(f),
        }
    }
}

impl From<Value> for Term {
    fn from(v: Value) -> Term {
        match v {
            Value::True(tru) => tru.into_term().into(),
            Value::False(fls) => fls.into_term().into(),
            Value::Num(num) => num.into_term().into(),
        }
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
