use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
use errors::{check_error::CheckError, NotASubtype};
use syntax::{
    env::Environment,
    kinds::Kind,
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
            Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind).into())
        } else {
            Err(NotASubtype::new(self.clone(), sup.clone()).into())
        }
    }
}

impl<Lang> Kindcheck for Top<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Kind, CheckError> {
        Ok(self.kind.clone())
    }
}

impl<Lang> Normalize for Top<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn normalize(self, _: Environment<Self::Lang>) -> <Self::Lang as Language>::Type {
        self.into()
    }
}
