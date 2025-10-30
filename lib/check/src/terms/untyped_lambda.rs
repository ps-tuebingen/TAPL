use crate::Typecheck;
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    env::Environment, language::Language, terms::UntypedLambda, types::Fun, untyped::Untyped,
};

impl<Lang> Typecheck for UntypedLambda<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Fun<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
    Untyped<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        Ok(TypingDerivation::untyped_lambda(TypingConclusion::new(
            env,
            self.clone(),
            Untyped::new().into(),
        ))
        .into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
