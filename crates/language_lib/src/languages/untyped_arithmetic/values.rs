use super::terms::Term;
use derivation::latex::{LatexConfig, LatexFmt};
use eval::{
    errors::{ValueKind, ValueMismatch},
    values::{False, Num, True, Value as ValueTrait, ValueGroup},
};
use std::fmt;
use syntax::untyped::Untyped;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    True(True<Term>),
    False(False<Term>),
    Num(Num<Term>),
}

impl ValueTrait for Value {
    type Term = Term;
    fn knd(&self) -> ValueKind {
        match self {
            Value::True(t) => t.knd(),
            Value::False(f) => f.knd(),
            Value::Num(n) => n.knd(),
        }
    }
}

impl ValueGroup for Value {
    type Term = Term;
    type Type = Untyped;

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

    fn into_num(self) -> Result<Num<Term>, ValueMismatch> {
        if let Value::Num(num) = self {
            Ok(num)
        } else {
            Err(ValueMismatch::new(&self, ValueKind::Number))
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

impl LatexFmt for Value {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Value::True(tru) => tru.to_latex(conf),
            Value::False(fls) => fls.to_latex(conf),
            Value::Num(num) => num.to_latex(conf),
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
