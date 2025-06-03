use crate::{Kindcheck, Normalize};
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{Exists, Type},
};

impl<Ty> Kindcheck<Ty> for Exists<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    type CheckError = <Ty as Kindcheck<Ty>>::CheckError;

    fn check_kind(&self, env: &mut Environment<Ty>) -> Result<Kind, Self::CheckError> {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        self.ty.check_kind(env)
    }
}

impl<Ty> Normalize<Ty> for Exists<Ty>
where
    Ty: Type + Normalize<Ty>,
    Self: Into<Ty>,
{
    fn normalize(self, env: &mut Environment<Ty>) -> Ty {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        self.into()
    }
}
