use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    kinds::Kind,
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
        sup: &<Self::Lang as Language>::Type,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        if let Ok(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind).into());
        }

        sup.clone().into_bool()?;
        Ok(SubtypeDerivation::refl(env, self.clone()).into())
    }
}

impl<Lang> Kindcheck for Bool<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Kind, CheckError> {
        Ok(Kind::Star)
    }
}

impl<Lang> Normalize for Bool<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn normalize(self, _: Environment<Self::Lang>) -> <Lang as Language>::Type {
        self.into()
    }
}
