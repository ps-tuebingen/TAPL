use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, KindingDerivation, NormalizingDerivation, SubtypeDerivation};
use errors::{NotASubtype, check_error::CheckError};
use grammar::{DerivationRule, symbols::SpecialChar};
use std::collections::HashSet;
use syntax::{
    env::Environment,
    language::Language,
    types::{Top, TypeGroup},
};

impl<Lang> Subtypecheck for Top<Lang>
where
    Lang: Language,
    Top<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    fn check_subtype(
        &self,
        sup: &<Lang as Language>::Type,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        if let Ok(top) = sup.clone().into_top() {
            Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind, vec![]).into())
        } else {
            Err(NotASubtype::new(self.clone(), sup.clone()).into())
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::sub_top()])
    }
}

impl<Lang> Kindcheck for Top<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Derivation<Lang>, CheckError> {
        Ok(KindingDerivation::annotated(self.clone(), self.kind.clone()).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::kind_any(SpecialChar::Top.into())])
    }
}

impl<Lang> Normalize for Top<Lang>
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
