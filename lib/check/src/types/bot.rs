use crate::{Kindcheck, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, kinds::Kind, language::Language, types::Bot};

impl<Lang> Subtypecheck for Bot<Lang>
where
    Lang: Language,
    Bot<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    fn check_subtype(
        &self,
        sup: &<Lang as Language>::Type,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        Ok(SubtypeDerivation::sup_bot(env, sup.clone()).into())
    }
}

impl<Lang> Kindcheck for Bot<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    fn check_kind(&self, _: Environment<Self::Lang>) -> Result<Kind, CheckError> {
        Ok(self.kind.clone())
    }
}
