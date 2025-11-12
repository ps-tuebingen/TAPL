use super::{UntypedLambda, terms::Term};
use macros::{FromVariants, GrammarDescribe, IntoTerm, LangDisplay, LatexFmt};
use syntax::values::{UntypedLambda as UntypedLambdaVal, Value as ValueTrait, ValueGroup};

#[derive(
    FromVariants, IntoTerm, GrammarDescribe, LatexFmt, LangDisplay, Debug, Clone, PartialEq, Eq,
)]
#[Lang(UntypedLambda)]
pub enum Value {
    Lambda(UntypedLambdaVal<UntypedLambda>),
}

impl ValueTrait for Value {
    type Lang = UntypedLambda;
    type Term = Term;
}

impl ValueGroup for Value {}
