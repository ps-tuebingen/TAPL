use super::{terms::Term, types::Type};
use common::errors::ErrorKind;
use eval::values::{
    Exception, False, Lambda, Num, Raise, True, Unit, Value as ValueTrait, ValueGroup,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(Lambda<Term, Type>),
    Num(Num<Term>),
    Unit(Unit<Term>),
    True(True<Term>),
    False(False<Term>),
    Exception(Exception<Term, Type>),
    Raise(Raise<Value, Type>),
}

impl eval::values::Value for Value {
    type Term = Term;
}

impl ValueGroup for Value {
    type Term = Term;
    type Type = Type;

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

    fn into_exception(self) -> Result<Exception<Term, Type>, ErrorKind> {
        if let Value::Exception(ex) = self {
            Ok(ex)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Exception".to_owned(),
            })
        }
    }

    fn into_lambda(self) -> Result<Lambda<Term, Type>, ErrorKind> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Lambda Abstraction".to_owned(),
            })
        }
    }

    fn into_raise(self) -> Result<Raise<Term, Type>, ErrorKind> {
        if let Value::Raise(raise) = self {
            Ok(raise)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Raise".to_owned(),
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

impl From<Exception<Term, Type>> for Value {
    fn from(exc: Exception<Term, Type>) -> Value {
        Value::Exception(exc)
    }
}

impl From<Raise<Term, Type>> for Value {
    fn from(raise: Raise<Term, Type>) -> Value {
        Value::Raise(raise)
    }
}
