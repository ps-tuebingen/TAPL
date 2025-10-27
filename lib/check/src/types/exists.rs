use crate::{Kindcheck, Normalize};
use derivations::{Derivation, KindingDerivation, NormalizingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, types::Exists};

impl<Lang> Kindcheck for Exists<Lang>
where
    Lang: Language,
    Lang::Type: Kindcheck<Lang = Lang>,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn check_kind(&self, mut env: Environment<Self::Lang>) -> Result<Derivation<Lang>, CheckError> {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        let ty_res = self.ty.check_kind(env)?.into_kind()?;
        Ok(KindingDerivation::exists(self.clone(), ty_res.ret_kind(), ty_res).into())
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
