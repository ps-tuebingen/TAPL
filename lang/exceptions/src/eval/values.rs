use crate::{syntax::Term, types::Type};
use common::{
    errors::ErrorKind,
    values::{Exception, False, Lambda, Num, Raise, True, Unit, Value as ValueTrait},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(Lambda<Term, Type>),
    Num(Num),
    Unit(Unit),
    True(True),
    False(False),
    Exception(Exception<Type>),
    Raise(Raise<Value, Type, Term>),
}

impl ValueTrait<Term> for Value {
    type Term = Term;

    fn into_lambda(self) -> Result<Lambda<Term, Type>, ErrorKind> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ErrorKind::TypeMismatch {
                found: self.to_string(),
                expected: "Lambda Abstraction".to_owned(),
            })
        }
    }

    fn into_raise(self) -> Result<Raise<Value, Type, Term>, ErrorKind> {
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

impl From<Lambda<Term, Type>> for Value {
    fn from(lam: Lambda<Term, Type>) -> Value {
        Value::Lambda(lam)
    }
}

impl From<Num> for Value {
    fn from(num: Num) -> Value {
        Value::Num(num)
    }
}

impl From<Unit> for Value {
    fn from(unit: Unit) -> Value {
        Value::Unit(unit)
    }
}

impl From<True> for Value {
    fn from(tru: True) -> Value {
        Value::True(tru)
    }
}

impl From<False> for Value {
    fn from(fls: False) -> Value {
        Value::False(fls)
    }
}

impl From<Exception<Type>> for Value {
    fn from(exc: Exception<Type>) -> Value {
        Value::Exception(exc)
    }
}

impl From<Raise<Value, Type, Term>> for Value {
    fn from(raise: Raise<Value, Type, Term>) -> Value {
        Value::Raise(raise)
    }
}
