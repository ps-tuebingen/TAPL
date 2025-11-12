use super::{UntypedLambda, terms::Term};
use macros::{GrammarDescribe, LangDisplay, LatexFmt};
use syntax::values::{UntypedLambda as UntypedLambdaVal, Value as ValueTrait, ValueGroup};

#[derive(GrammarDescribe, LatexFmt, LangDisplay, Debug, Clone, PartialEq, Eq)]
#[Lang(UntypedLambda)]
pub enum Value {
    Lambda(UntypedLambdaVal<UntypedLambda>),
}

impl ValueTrait for Value {
    type Lang = UntypedLambda;
    type Term = Term;
}

impl ValueGroup for Value {}

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
