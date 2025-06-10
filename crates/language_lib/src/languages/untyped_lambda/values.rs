use super::terms::Term;
use common::errors::{ValueKind, ValueMismatch};
use derivation::latex::{LatexConfig, LatexFmt};
use syntax::untyped::Untyped;
use syntax::values::{Lambda, Value as ValueTrait, ValueGroup};

use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(Lambda<Term, Untyped>),
}

impl ValueTrait for Value {
    type Term = Term;
    fn knd(&self) -> ValueKind {
        match self {
            Value::Lambda(l) => l.knd(),
        }
    }
}

impl ValueGroup for Value {
    type Term = Term;
    type Type = Untyped;

    fn into_lambda(self) -> Result<Lambda<Term, Untyped>, ValueMismatch> {
        match self {
            Value::Lambda(lam) => Ok(lam),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Lambda(lam) => lam.fmt(f),
        }
    }
}

impl LatexFmt for Value {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Value::Lambda(lam) => lam.to_latex(conf),
        }
    }
}

impl From<Value> for Term {
    fn from(val: Value) -> Term {
        match val {
            Value::Lambda(lam) => lam.into_term().into(),
        }
    }
}

impl From<Lambda<Term, Untyped>> for Value {
    fn from(lam: Lambda<Term, Untyped>) -> Value {
        Value::Lambda(lam)
    }
}
