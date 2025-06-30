use super::terms::Term;
use common::errors::ValueKind;
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use syntax::untyped::Untyped;
use syntax::values::{UntypedLambda, Value as ValueTrait, ValueGroup};

use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(UntypedLambda<Term>),
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
}

impl GrammarDescribe for Value {
    fn grammar() -> Grammar {
        Grammar::value(vec![UntypedLambda::<Term>::rule()])
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

impl From<UntypedLambda<Term>> for Value {
    fn from(lam: UntypedLambda<Term>) -> Value {
        Value::Lambda(lam)
    }
}
