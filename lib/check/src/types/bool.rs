use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, KindingDerivation, NormalizingDerivation, SubtypeDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    language::Language,
    types::{Bool, Top, TypeGroup},
};

impl<Lang> Subtypecheck for Bool<Lang>
where
    Lang: Language,
    Top<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Type>,
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

        sup.clone().into_bool()?;
        Ok(SubtypeDerivation::refl(env, self.clone(), vec![]).into())
    }
}

impl<Lang> Kindcheck for Bool<Lang>
where
    Lang: Language,
    Self: Into<<Lang as Language>::Type>,
{
    type Lang = Lang;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        Ok(KindingDerivation::prim(self.clone()).into())
    }
}

impl<Lang> Normalize for Bool<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn normalize(self, _: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        NormalizingDerivation::empty(self).into()
    }
}
