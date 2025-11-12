use super::UntypedLambda;
use check::Typecheck;
use derivations::Derivation;
use errors::{NoTyping, check_error::CheckError};
use grammar::DerivationRule;
use macros::{Eval, FromVariants, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, SubstType};
use std::collections::HashSet;
use syntax::{
    env::Environment,
    language::Language,
    terms::{App, UntypedLambda as UntypedLambdaT, Variable},
};

#[derive(
    FromVariants,
    SubstType,
    SubstTerm,
    LatexFmt,
    LangDisplay,
    GrammarDescribe,
    Eval,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
#[Lang(UntypedLambda)]
pub enum Term {
    Var(Variable<UntypedLambda>),
    Lambda(UntypedLambdaT<UntypedLambda>),
    App(App<UntypedLambda>),
}

impl syntax::terms::Term for Term {}

impl Typecheck for Term {
    type Lang = UntypedLambda;

    fn check(&self, _: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoTyping::new(UntypedLambda.describe()).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
