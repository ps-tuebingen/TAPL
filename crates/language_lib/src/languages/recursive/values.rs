use super::{terms::Term, types::Type};
use common::errors::ErrorKind;
use eval::values::{
    False, Fold, Lambda, Num, Pair, Record, True, Unit, Value as ValueTrait, ValueGroup, Variant,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    True(True<Term>),
    False(False<Term>),
    Unit(Unit<Term>),
    Num(Num<Term>),
    Lambda(Lambda<Term, Type>),
    Fold(Fold<Value, Type>),
    Pair(Pair<Value>),
    Record(Record<Value>),
    Variant(Variant<Value, Type>),
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

    fn into_fold(self) -> Result<Fold<Value, Type>, ErrorKind> {
        if let Value::Fold(fld) = self {
            Ok(fld)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Fold".to_owned(),
            })
        }
    }

    fn into_pair(self) -> Result<Pair<Value>, ErrorKind> {
        if let Value::Pair(pair) = self {
            Ok(pair)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Pair".to_owned(),
            })
        }
    }

    fn into_record(self) -> Result<Record<Value>, ErrorKind> {
        if let Value::Record(rec) = self {
            Ok(rec)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Record".to_owned(),
            })
        }
    }

    fn into_variant(self) -> Result<Variant<Value, Type>, ErrorKind> {
        if let Value::Variant(var) = self {
            Ok(var)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Variant".to_owned(),
            })
        }
    }
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

impl From<Fold<Value, Type>> for Value {
    fn from(fld: Fold<Value, Type>) -> Value {
        Value::Fold(fld)
    }
}

impl From<Lambda<Term, Type>> for Value {
    fn from(lam: Lambda<Term, Type>) -> Value {
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
impl From<Pair<Value>> for Value {
    fn from(pair: Pair<Value>) -> Value {
        Value::Pair(pair)
    }
}

impl From<Record<Value>> for Value {
    fn from(rec: Record<Value>) -> Value {
        Value::Record(rec)
    }
}

impl From<Variant<Value, Type>> for Value {
    fn from(var: Variant<Value, Type>) -> Value {
        Value::Variant(var)
    }
}
