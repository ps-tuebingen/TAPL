use crate::terms::Term;
use common::{
    language::LanguageValue,
    values::{Lambda, Loc, Num, Unit, Value as ValueTrait},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(Lambda<Term>),
    Unit(Unit<Term>),
    Num(Num<Term>),
    Loc(Loc<Term>),
}

impl ValueTrait for Value {
    type Term = Term;
}

impl LanguageValue for Value {
    type Term = Term;
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda(lam) => lam.into_term().into(),
            Value::Num(num) => num.into_term().into(),
            Value::Unit(u) => u.into_term().into(),
            Value::Loc(loc) => loc.into_term().into(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Lambda(lam) => lam.fmt(f),
            Value::Num(num) => num.fmt(f),
            Value::Unit(u) => u.fmt(f),
            Value::Loc(loc) => loc.fmt(f),
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
impl From<Num<Term>> for Value {
    fn from(n: Num<Term>) -> Value {
        Value::Num(n)
    }
}
impl From<Loc<Term>> for Value {
    fn from(loc: Loc<Term>) -> Value {
        Value::Loc(loc)
    }
}
