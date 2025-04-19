use super::terms::Term;
use common::{
    language::LanguageValue,
    values::{False, Lambda, Num, Pack, Record, True, TyLambda, Unit, Value as ValueTrait},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(Lambda<Term>),
    TyLambda(TyLambda<Term>),
    Pack(Pack<Term>),
    Record(Record<Term>),
    True(True<Term>),
    False(False<Term>),
    Unit(Unit<Term>),
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
            Value::TyLambda(tylam) => tylam.into_term().into(),
            Value::Pack(pack) => pack.into_term().into(),
            Value::Record(rec) => rec.into_term().into(),
            Value::True(tru) => tru.into_term().into(),
            Value::False(fls) => fls.into_term().into(),
            Value::Unit(u) => u.into_term().into(),
            Value::Num(num) => num.into_term().into(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Lambda(lam) => lam.fmt(f),
            Value::TyLambda(tylam) => tylam.fmt(f),
            Value::Pack(pack) => pack.fmt(f),
            Value::Record(rec) => rec.fmt(f),
            Value::True(tru) => tru.fmt(f),
            Value::False(fls) => fls.fmt(f),
            Value::Unit(u) => u.fmt(f),
            Value::Num(num) => num.fmt(f),
        }
    }
}
