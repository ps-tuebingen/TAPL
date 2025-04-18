use super::terms::Term;
use common::{
    language::LanguageValue,
    values::{
        Cons, False, Lambda, Loc, Nil, Num, Record, True, Unit, Value as ValueTrait, Variant,
    },
};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Lambda(Lambda<Term>),
    Unit(Unit<Term>),
    Record(Record<Term>),
    Variant(Variant<Term>),
    Nil(Nil<Term>),
    Cons(Cons<Term>),
    Loc(Loc<Term>),
    Num(Num<Term>),
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
            Value::Lambda(lam) => lam.into_term().into(),
            Value::Unit(u) => u.into_term().into(),
            Value::Record(rec) => rec.into_term().into(),
            Value::Variant(var) => var.into_term().into(),
            Value::Nil(nil) => nil.into_term().into(),
            Value::Cons(cons) => cons.into_term().into(),
            Value::Loc(loc) => loc.into_term().into(),
            Value::Num(num) => num.into_term().into(),
            Value::True(tru) => tru.into_term().into(),
            Value::False(fls) => fls.into_term().into(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Lambda(lam) => lam.fmt(f),
            Value::Unit(u) => u.fmt(f),
            Value::Record(rec) => rec.fmt(f),
            Value::Variant(var) => var.fmt(f),
            Value::Nil(nil) => nil.fmt(f),
            Value::Cons(cons) => cons.fmt(f),
            Value::Loc(loc) => loc.fmt(f),
            Value::Num(num) => num.fmt(f),
            Value::True(tru) => tru.fmt(f),
            Value::False(fls) => fls.fmt(f),
        }
    }
}

impl From<Loc<Term>> for Value {
    fn from(loc: Loc<Term>) -> Value {
        Value::Loc(loc)
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

impl From<Variant<Term>> for Value {
    fn from(var: Variant<Term>) -> Value {
        Value::Variant(var)
    }
}

impl From<Nil<Term>> for Value {
    fn from(nil: Nil<Term>) -> Value {
        Value::Nil(nil)
    }
}
impl From<Cons<Term>> for Value {
    fn from(cons: Cons<Term>) -> Value {
        Value::Cons(cons)
    }
}
