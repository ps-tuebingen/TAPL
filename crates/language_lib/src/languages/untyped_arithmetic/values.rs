use super::terms::Term;
use common::errors::ErrorKind;
use eval::values::{False, Num, True, Value as ValueTrait, ValueGroup};
use std::fmt;
use syntax::untyped::Untyped;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    True(True<Term>),
    False(False<Term>),
    Num(Num<Term>),
}

impl eval::values::Value for Value {
    type Term = Term;
}

impl ValueGroup for Value {
    type Term = Term;
    type Type = Untyped;

    fn into_true(self) -> Result<True<Term>, ErrorKind> {
        if let Value::True(tru) = self {
            Ok(tru)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "True".to_owned(),
            })
        }
    }

    fn into_false(self) -> Result<False<Term>, ErrorKind> {
        if let Value::False(fls) = self {
            Ok(fls)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "False".to_owned(),
            })
        }
    }

    fn into_num(self) -> Result<Num<Term>, ErrorKind> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Number".to_owned(),
            })
        }
    }
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
