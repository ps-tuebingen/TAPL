use crate::{errors::CheckError, Kindcheck, Normalize};
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{Exists, Type},
};

impl<Ty> Kindcheck<Ty> for Exists<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    fn check_kind(&self, mut env: Environment<Ty>) -> Result<Kind, CheckError> {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        self.ty.check_kind(env)
    }
}

impl<Ty> Normalize<Ty> for Exists<Ty>
where
    Ty: Type + Normalize<Ty>,
    Self: Into<Ty>,
{
    fn normalize(self, mut env: Environment<Ty>) -> Ty {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        self.into()
    }
}
