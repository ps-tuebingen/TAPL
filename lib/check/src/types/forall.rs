use crate::{Kindcheck, Normalize};
use errors::check_error::CheckError;
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
    fn normalize(self, mut env: Environment<Self::Lang>) -> <Self::Lang as Language>::Type {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        let ty_norm = self.ty.normalize(env);
        Forall {
            var: self.var,
            kind: self.kind,
            ty: Box::new(ty_norm),
        }
        .into()
    }
}
