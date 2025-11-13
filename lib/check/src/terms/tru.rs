use crate::Typecheck;
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use grammar::{DerivationRule, symbols::Keyword};
use std::collections::HashSet;
use syntax::{env::Environment, language::Language, terms::True, types::Bool};

impl<Lang> Typecheck for True<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Bool<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let conc = TypingConclusion::new(env, self.clone(), Bool::new());
        let deriv = TypingDerivation::tru(conc);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_const(
            Keyword::True,
            Keyword::Bool,
            "T-True",
        )])
    }
}
