use super::terms::Term;
use common::{
    language::LanguageValue,
    values::{Lambda, LambdaSub, Num, Pack, Record, Value as ValueTrait},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(Lambda<Term>),
    LambdaSub(LambdaSub<Term>),
    Pack(Pack<Term>),
    Record(Record<Term>),
    Num(Num<Term>),
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
            Value::LambdaSub(lam) => lam.into_term().into(),
            Value::Pack(pack) => pack.into_term().into(),
            Value::Record(rec) => rec.into_term().into(),
            Value::Num(num) => num.into_term().into(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Lambda(lam) => lam.fmt(f),
            Value::LambdaSub(lam) => lam.fmt(f),
            Value::Pack(pack) => pack.fmt(f),
            Value::Record(rec) => rec.fmt(f),
            Value::Num(num) => num.fmt(f),
        }
    }
}

impl From<Pack<Term>> for Value {
    fn from(pack: Pack<Term>) -> Value {
        Value::Pack(pack)
    }
}
impl From<LambdaSub<Term>> for Value {
    fn from(lam: LambdaSub<Term>) -> Value {
        Value::LambdaSub(lam)
    }
}
impl From<Lambda<Term>> for Value {
    fn from(lam: Lambda<Term>) -> Value {
        Value::Lambda(lam)
    }
}
impl From<Num<Term>> for Value {
    fn from(num: Num<Term>) -> Value {
        Value::Num(num)
    }
}

impl From<Record<Term>> for Value {
    fn from(rec: Record<Term>) -> Value {
        Value::Record(rec)
    }
}
