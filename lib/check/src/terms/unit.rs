use crate::Typecheck;
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use grammar::{DerivationRule, symbols::Keyword};
use std::collections::HashSet;
use syntax::{env::Environment, language::Language, terms::Unit, types::Unit as UnitTy};

impl<Lang> Typecheck for Unit<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    UnitTy<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let conc = TypingConclusion::new(env.clone(), self.clone(), UnitTy::<Lang>::new());
        let deriv = TypingDerivation::unit(conc);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_const(
            Keyword::Unit,
            Keyword::Unit,
            "T-Unit",
        )])
    }
}
