use crate::{errors::CheckError, Kindcheck, Normalize};
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{Forall, Type},
};

impl<Ty> Kindcheck<Ty> for Forall<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    fn check_kind(&self, mut env: Environment<Ty>) -> Result<Kind, CheckError> {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        let ty_kind = self.ty.check_kind(env)?;
        Ok(ty_kind)
    }
}

impl<Ty> Normalize<Ty> for Forall<Ty>
where
    Ty: Type + Normalize<Ty>,
    Self: Into<Ty>,
{
    fn normalize(self, mut env: Environment<Ty>) -> Ty {
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
