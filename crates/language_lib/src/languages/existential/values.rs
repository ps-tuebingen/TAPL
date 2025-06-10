use super::{terms::Term, types::Type};
use common::errors::{ValueKind, ValueMismatch};
use derivation::latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::values::{
    False, Lambda, Num, Pack, Record, True, Unit, Value as ValueTrait, ValueGroup,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Unit(Unit<Term>),
    Lambda(Lambda<Term, Type>),
    Pack(Pack<Value, Type>),
    Num(Num<Term>),
    Record(Record<Value>),
    True(True<Term>),
    False(False<Term>),
}

impl ValueTrait for Value {
    type Term = Term;
    fn knd(&self) -> ValueKind {
        match self {
            Value::Unit(v) => v.knd(),
            Value::Lambda(v) => v.knd(),
            Value::Pack(v) => v.knd(),
            Value::Num(v) => v.knd(),
            Value::Record(v) => v.knd(),
            Value::True(v) => v.knd(),
            Value::False(v) => v.knd(),
        }
    }
}

impl ValueGroup for Value {
    type Term = Term;
    type Type = Type;

    fn into_lambda(self) -> Result<Lambda<Term, Type>, ValueMismatch> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::Lambda))
        }
    }

    fn into_pack(self) -> Result<Pack<Value, Type>, ValueMismatch> {
        if let Value::Pack(pack) = self {
            Ok(pack)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::Package))
        }
    }

    fn into_num(self) -> Result<Num<Term>, ValueMismatch> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::Number))
        }
    }

    fn into_record(self) -> Result<Record<Value>, ValueMismatch> {
        if let Value::Record(rec) = self {
            Ok(rec)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::Record))
        }
    }

    fn into_true(self) -> Result<True<Term>, ValueMismatch> {
        if let Value::True(tru) = self {
            Ok(tru)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::True))
        }
    }

    fn into_false(self) -> Result<False<Term>, ValueMismatch> {
        if let Value::False(fls) = self {
            Ok(fls)
        } else {
            Err(ValueMismatch::new(self.knd(), ValueKind::False))
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

impl LatexFmt for Value {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Value::Unit(u) => u.to_latex(conf),
            Value::Lambda(lam) => lam.to_latex(conf),
            Value::Pack(pack) => pack.to_latex(conf),
            Value::Num(num) => num.to_latex(conf),
            Value::Record(rec) => rec.to_latex(conf),
            Value::True(tru) => tru.to_latex(conf),
            Value::False(fls) => fls.to_latex(conf),
        }
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
impl From<Record<Value>> for Value {
    fn from(rec: Record<Value>) -> Value {
        Value::Record(rec)
    }
}

impl From<Pack<Value, Type>> for Value {
    fn from(pack: Pack<Value, Type>) -> Value {
        Value::Pack(pack)
    }
}
