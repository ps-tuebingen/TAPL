use super::{UntypedLambda, terms::Term};
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use macros::{LangDisplay, LatexFmt};
use syntax::values::{UntypedLambda as UntypedLambdaVal, Value as ValueTrait, ValueGroup};

#[derive(LatexFmt, LangDisplay, Debug, Clone, PartialEq, Eq)]
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
