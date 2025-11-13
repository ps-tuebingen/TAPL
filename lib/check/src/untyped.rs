use super::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, NormalizingDerivation};
use errors::{NoKinding, NoSubtyping, check_error::CheckError};
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{env::Environment, language::Language, untyped::Untyped};

impl<Lang> Kindcheck for Untyped<Lang>
where
    Lang: Language,
{
    type Lang = Lang;

    fn check_kind(&self, _: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        Err(NoKinding::new("Untyped").into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}

impl<Lang> Normalize for Untyped<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn normalize(self, _: Environment<Lang>) -> Derivation<Self::Lang> {
        NormalizingDerivation::empty(Self::new()).into()
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}

impl<Lang> Subtypecheck for Untyped<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    fn check_subtype(
        &self,
        _: &<Self::Lang as Language>::Type,
        _: Environment<Lang>,
    ) -> Result<Derivation<Lang>, CheckError> {
        Err(NoSubtyping::new("Untyped").into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
