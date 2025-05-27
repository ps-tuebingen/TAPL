use crate::{env::CheckEnvironment, Kindcheck};
use common::errors::Error;
use syntax::{
    kinds::Kind,
    types::{Exists, Type},
};

impl<Ty> Kindcheck<Ty> for Exists<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;

    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error> {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        self.ty.check_kind(env)
    }
}

impl<Ty> Normalize<Ty> for Exists<Ty>
where
    Ty: TypeGroup,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        self.into()
    }
}
