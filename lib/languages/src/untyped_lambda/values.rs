use super::{UntypedLambda, terms::Term};
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use syntax::values::{UntypedLambda as UntypedLambdaVal, Value as ValueTrait, ValueGroup};

use std::fmt;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Lambda(UntypedLambdaVal<UntypedLambda>),
}

impl ValueTrait for Value {
    type Lang = UntypedLambda;
    type Term = Term;
}

impl ValueGroup for Value {}

impl GrammarDescribe for Value {
    fn grammar() -> Grammar {
        Grammar::value(vec![UntypedLambdaVal::<UntypedLambda>::rule()])
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
            Value::Lambda(lam) => lam.into_term(),
        }
    }
}

impl From<UntypedLambdaVal<UntypedLambda>> for Value {
    fn from(lam: UntypedLambdaVal<UntypedLambda>) -> Value {
        Value::Lambda(lam)
    }
}
