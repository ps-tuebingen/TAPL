use super::{terms::Term, types::Type};
use derivation::latex::{LatexConfig, LatexFmt};
use eval::{
    errors::{ValueKind, ValueMismatch},
    values::{False, Lambda, Loc, Num, True, Unit, Value as ValueTrait, ValueGroup},
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(Lambda<Term, Type>),
    Unit(Unit<Term>),
    Num(Num<Term>),
    Loc(Loc<Term>),
    True(True<Term>),
    False(False<Term>),
}

impl ValueTrait for Value {
    type Term = Term;

    fn knd(&self) -> ValueKind {
        match self {
            Value::Lambda(v) => v.knd(),
            Value::Unit(v) => v.knd(),
            Value::Num(v) => v.knd(),
            Value::Loc(v) => v.knd(),
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
            Err(ValueMismatch::new(&self, ValueKind::Lambda))
        }
    }

    fn into_num(self) -> Result<Num<Term>, ValueMismatch> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(&self, ValueKind::Number))
        }
    }

    fn into_loc(self) -> Result<Loc<Term>, ValueMismatch> {
        if let Value::Loc(loc) = self {
            Ok(loc)
        } else {
            Err(ValueMismatch::new(&self, ValueKind::Location))
        }
    }

    fn into_true(self) -> Result<True<Term>, ValueMismatch> {
        if let Value::True(tru) = self {
            Ok(tru)
        } else {
            Err(ValueMismatch::new(&self, ValueKind::True))
        }
    }

    fn into_false(self) -> Result<False<Term>, ValueMismatch> {
        if let Value::False(fls) = self {
            Ok(fls)
        } else {
            Err(ValueMismatch::new(&self, ValueKind::False))
        }
    }
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda(lam) => lam.into_term().into(),
            Value::Num(num) => num.into_term().into(),
            Value::Unit(u) => u.into_term().into(),
            Value::Loc(loc) => loc.into_term().into(),
            Value::True(tru) => tru.into_term().into(),
            Value::False(fls) => fls.into_term().into(),
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
            Value::True(tru) => tru.fmt(f),
            Value::False(fls) => fls.fmt(f),
        }
    }
}

impl LatexFmt for Value {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Value::Lambda(lam) => lam.to_latex(conf),
            Value::Num(num) => num.to_latex(conf),
            Value::Unit(u) => u.to_latex(conf),
            Value::Loc(loc) => loc.to_latex(conf),
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
