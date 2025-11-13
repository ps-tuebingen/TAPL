use crate::{Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use grammar::{DerivationRule, symbols::Keyword};
use std::collections::HashSet;
use syntax::{
    env::Environment,
    language::Language,
    terms::False,
    types::{Bool, Type},
};

impl<Lang> Typecheck for False<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type: Type + Normalize<Lang = Lang>,
    Bool<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let conc = TypingConclusion::new(env, self.clone(), Bool::new());
        Ok(TypingDerivation::fls(conc).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_const(
            Keyword::False,
            Keyword::Bool,
            "T-False",
        )])
    }
}
