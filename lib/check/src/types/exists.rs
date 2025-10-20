use crate::{Kindcheck, Normalize};
use derivations::{Derivation, NormalizingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, kinds::Kind, language::Language, types::Exists};

impl<Lang> Kindcheck for Exists<Lang>
where
    Lang: Language,
    Lang::Type: Kindcheck<Lang = Lang>,
{
    type Lang = Lang;
    fn check_kind(&self, mut env: Environment<Self::Lang>) -> Result<Kind, CheckError> {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        self.ty.check_kind(env)
    }
}

impl<Lang> Normalize for Exists<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn normalize(self, mut env: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        NormalizingDerivation::empty(self).into()
    }
}
