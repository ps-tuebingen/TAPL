use super::terms::Term;
use common::{
    language::LanguageValue,
    values::{False, Fold, Lambda, Num, Pair, Record, True, Unit, Value as ValueTrait, Variant},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    True(True<Term>),
    False(False<Term>),
    Unit(Unit<Term>),
    Num(Num<Term>),
    Lambda(Lambda<Term>),
    Fold(Fold<Term>),
    Pair(Pair<Term>),
    Record(Record<Term>),
    Variant(Variant<Term>),
}

impl common::values::Value for Value {
    type Term = Term;
}

impl LanguageValue for Value {
    type Term = Term;
}

impl From<Value> for Term {
    fn from(v: Value) -> Term {
        match v {
            Value::Unit(u) => u.into_term().into(),
            Value::True(tru) => tru.into_term().into(),
            Value::False(fls) => fls.into_term().into(),
            Value::Num(num) => num.into_term().into(),
            Value::Lambda(lam) => lam.into_term().into(),
            Value::Fold(fld) => fld.into_term().into(),
            Value::Pair(pr) => pr.into_term().into(),
            Value::Record(rec) => rec.into_term().into(),
            Value::Variant(var) => var.into_term().into(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Unit(u) => u.fmt(f),
            Value::True(tru) => tru.fmt(f),
            Value::False(fls) => fls.fmt(f),
            Value::Num(num) => num.fmt(f),
            Value::Lambda(lam) => lam.fmt(f),
            Value::Fold(fld) => fld.fmt(f),
            Value::Pair(pr) => pr.fmt(f),
            Value::Record(rec) => rec.fmt(f),
            Value::Variant(var) => var.fmt(f),
        }
    }
}

impl From<Fold<Term>> for Value {
    fn from(fld: Fold<Term>) -> Value {
        Value::Fold(fld)
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
impl From<Pair<Term>> for Value {
    fn from(pair: Pair<Term>) -> Value {
        Value::Pair(pair)
    }
}

impl From<Record<Term>> for Value {
    fn from(rec: Record<Term>) -> Value {
        Value::Record(rec)
    }
}

impl From<Variant<Term>> for Value {
    fn from(var: Variant<Term>) -> Value {
        Value::Variant(var)
    }
}
