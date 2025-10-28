use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, KindingDerivation, NormalizingDerivation, SubtypeDerivation};
use errors::check_error::CheckError;
use grammar::{DerivationRule, symbols::SpecialChar};
use std::collections::HashSet;
use syntax::{
    env::Environment,
    language::Language,
    types::{Top, TypeGroup, Unit},
};

impl<Lang> Kindcheck for Unit<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Derivation<Lang>, CheckError> {
        Ok(KindingDerivation::prim(self.clone()).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::kind_prim(SpecialChar::Star.into())])
    }
}

impl<Lang> Subtypecheck for Unit<Lang>
where
    Lang: Language,
    Top<Lang>: Into<Lang::Type>,
    Unit<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    fn check_subtype(
        &self,
        sup: &<Lang as Language>::Type,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        if let Ok(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind, vec![]).into());
        }

        sup.clone().into_unit()?;
        Ok(SubtypeDerivation::refl(env, self.clone(), vec![]).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::sub_top(), DerivationRule::sub_refl()])
    }
}

impl<Lang> Normalize for Unit<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn normalize(self, _: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        NormalizingDerivation::empty(self).into()
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
