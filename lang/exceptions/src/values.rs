use crate::terms::Term;
use common::{
    errors::ErrorKind,
    language::LanguageValue,
    values::{Exception, False, Lambda, Num, Raise, True, Unit, Value as ValueTrait},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(Lambda<Term>),
    Num(Num<Term>),
    Unit(Unit<Term>),
    True(True<Term>),
    False(False<Term>),
    Exception(Exception<Term>),
    Raise(Raise<Term>),
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
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Lambda Abstraction".to_owned(),
            })
        }
    }

    fn into_raise(self) -> Result<Raise<Term>, ErrorKind> {
        if let Value::Raise(raise) = self {
            Ok(raise)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Raise".to_owned(),
            })
        }
    }
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda(lam) => lam.into_term().into(),
            Value::Unit(u) => u.into_term().into(),
            Value::True(tru) => tru.into_term().into(),
            Value::False(fls) => fls.into_term().into(),
            Value::Num(num) => num.into_term().into(),
            Value::Exception(exc) => exc.into_term().into(),
            Value::Raise(raise) => raise.into_term().into(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Lambda(lam) => lam.fmt(f),
            Value::Unit(u) => u.fmt(f),
            Value::True(tru) => tru.fmt(f),
            Value::False(fls) => fls.fmt(f),
            Value::Num(num) => num.fmt(f),
            Value::Exception(exc) => exc.fmt(f),
            Value::Raise(raise) => raise.fmt(f),
        }
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

impl From<Unit<Term>> for Value {
    fn from(unit: Unit<Term>) -> Value {
        Value::Unit(unit)
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

impl From<Exception<Term>> for Value {
    fn from(exc: Exception<Term>) -> Value {
        Value::Exception(exc)
    }
}

impl From<Raise<Term>> for Value {
    fn from(raise: Raise<Term>) -> Value {
        Value::Raise(raise)
    }
}
