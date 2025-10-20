use crate::{Kindcheck, Normalize};
use derivations::{Derivation, NormalizingDerivation};
use errors::check_error::CheckError;
use std::rc::Rc;
use syntax::{env::Environment, kinds::Kind, language::Language, types::Forall};

impl<Lang> Kindcheck for Forall<Lang>
where
    Lang: Language,
    Lang::Type: Kindcheck<Lang = Lang>,
{
    type Lang = Lang;
    fn check_kind(&self, mut env: Environment<Self::Lang>) -> Result<Kind, CheckError> {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        let ty_kind = self.ty.check_kind(env)?;
        Ok(ty_kind)
    }
}

impl<Lang> Normalize for Forall<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
    Lang::Type: Normalize<Lang = Lang>,
{
    type Lang = Lang;
    fn normalize(self, mut env: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        let ty_norm = self.ty.clone().normalize(env);
        let self_norm = Forall {
            var: self.var.clone(),
            kind: self.kind.clone(),
            ty: Rc::new(ty_norm.ret_ty()),
        };
        NormalizingDerivation::cong(self, self_norm, vec![ty_norm]).into()
    }
}
