use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    kinds::Kind,
    language::Language,
    types::{Top, TypeGroup, Unit},
};
impl<Lang> Kindcheck for Unit<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Kind, CheckError> {
        Ok(Kind::Star)
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
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind).into());
        }

        sup.clone().into_unit()?;
        Ok(SubtypeDerivation::refl(env, self.clone()).into())
    }
}

impl<Lang> Normalize for Unit<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn normalize(self, _: Environment<Self::Lang>) -> <Self::Lang as Language>::Type {
        self.into()
    }
}
