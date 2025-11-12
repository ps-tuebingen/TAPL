use super::UntypedLambda;
use check::Typecheck;
use derivations::Derivation;
use errors::{NoTyping, check_error::CheckError};
use grammar::DerivationRule;
use macros::{Eval, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, SubstType};
use std::collections::HashSet;
use syntax::{
    env::Environment,
    language::Language,
    terms::{App, UntypedLambda as UntypedLambdaT, Variable},
};

#[derive(
    SubstType, SubstTerm, LatexFmt, LangDisplay, GrammarDescribe, Eval, Debug, Clone, PartialEq, Eq,
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

impl From<Variable<UntypedLambda>> for Term {
    fn from(var: Variable<UntypedLambda>) -> Term {
        Term::Var(var)
    }
}

impl From<UntypedLambdaT<UntypedLambda>> for Term {
    fn from(lam: UntypedLambdaT<UntypedLambda>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<App<UntypedLambda>> for Term {
    fn from(app: App<UntypedLambda>) -> Term {
        Term::App(app)
    }
}
