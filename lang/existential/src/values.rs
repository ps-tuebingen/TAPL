use super::terms::Term;
use common::{
    language::LanguageValue,
    values::{False, Lambda, Num, Pack, Record, True, Unit, Value as ValueTrait},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Unit(Unit<Term>),
    Lambda(Lambda<Term>),
    Pack(Pack<Term>),
    Num(Num<Term>),
    Record(Record<Term>),
    True(True<Term>),
    False(False<Term>),
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
            Value::Unit(u) => u.into_term().into(),
            Value::Lambda(lam) => lam.into_term().into(),
            Value::Pack(pack) => pack.into_term().into(),
            Value::Num(num) => num.into_term().into(),
            Value::Record(rec) => rec.into_term().into(),
            Value::True(tru) => tru.into_term().into(),
            Value::False(fls) => fls.into_term().into(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Unit(u) => u.fmt(f),
            Value::Lambda(lam) => lam.fmt(f),
            Value::Pack(pack) => pack.fmt(f),
            Value::Num(num) => num.fmt(f),
            Value::Record(rec) => rec.fmt(f),
            Value::True(tru) => tru.fmt(f),
            Value::False(fls) => fls.fmt(f),
        }
    }
}

impl From<Lambda<Term>> for Value {
    fn from(lam: Lambda<Term>) -> Value {
        Value::Lambda(lam)
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
impl From<Record<Term>> for Value {
    fn from(rec: Record<Term>) -> Value {
        Value::Record(rec)
    }
}

impl From<Pack<Term>> for Value {
    fn from(pack: Pack<Term>) -> Value {
        Value::Pack(pack)
    }
}
