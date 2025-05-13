use super::terms::Term;
use common::{
    errors::ErrorKind,
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
    fn into_lambda(self) -> Result<Lambda<Term>, ErrorKind> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Lambda Abstraction".to_owned(),
            })
        }
    }

    fn into_pack(self) -> Result<Pack<Term>, ErrorKind> {
        if let Value::Pack(pack) = self {
            Ok(pack)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Pack".to_owned(),
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

    fn into_record(self) -> Result<Record<Term>, ErrorKind> {
        if let Value::Record(rec) = self {
            Ok(rec)
        } else {
            Err(ErrorKind::ValueMismatch {
                found: self.to_string(),
                expected: "Record".to_owned(),
            })
        }
    }

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
