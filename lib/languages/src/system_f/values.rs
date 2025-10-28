use super::{SystemF, terms::Term};
use errors::ValueMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::values::{Lambda, TyLambda, Value as ValueTrait, ValueGroup};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value {
    Lambda(Lambda<SystemF>),
    TyLambda(TyLambda<SystemF>),
}

impl ValueTrait for Value {
    type Lang = SystemF;
    type Term = Term;
}

impl ValueGroup for Value {
    fn into_lambda(self) -> Result<Lambda<SystemF>, ValueMismatch> {
        if let Value::Lambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "Lambda".to_owned()))
        }
    }

    fn into_tylambda(self) -> Result<TyLambda<SystemF>, ValueMismatch> {
        if let Value::TyLambda(lam) = self {
            Ok(lam)
        } else {
            Err(ValueMismatch::new(self.to_string(), "TyLambda".to_owned()))
        }
    }
}

impl From<Value> for Term {
    fn from(v: Value) -> Term {
        match v {
            Value::Lambda(lam) => lam.into_term(),
            Value::TyLambda(tylam) => tylam.into_term(),
        }
    }
}

impl GrammarDescribe for Value {
    fn grammar() -> Grammar {
        Grammar::value(vec![Lambda::<SystemF>::rule(), TyLambda::<SystemF>::rule()])
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Lambda(lam) => lam.fmt(f),
            Value::TyLambda(tylam) => tylam.fmt(f),
        }
    }
}

impl LatexFmt for Value {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Value::Lambda(lam) => lam.to_latex(conf),
            Value::TyLambda(tylam) => tylam.to_latex(conf),
        }
    }
}

impl From<Lambda<SystemF>> for Value {
    fn from(lam: Lambda<SystemF>) -> Value {
        Value::Lambda(lam)
    }
}

impl From<TyLambda<SystemF>> for Value {
    fn from(tylam: TyLambda<SystemF>) -> Value {
        Value::TyLambda(tylam)
    }
}
