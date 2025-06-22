use crate::{errors::CheckError, Kindcheck, Normalize, Subtypecheck};
use common::errors::NameMismatch;
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{ExistsBounded, Type, TypeGroup},
};

impl<Ty> Subtypecheck<Ty> for ExistsBounded<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty> + Normalize<Ty>,
{
    fn check_subtype(&self, sup: &Ty, mut env: Environment<Ty>) -> Result<(), CheckError<Ty>> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        let sup_norm = sup.clone().normalize(env.clone());
        let self_norm = self.sup_ty.clone().normalize(env.clone());
        let other_exists = sup_norm.into_exists_bounded()?;
        other_exists.sup_ty.check_equal(&self_norm)?;
        if self.var != other_exists.var {
            return Err(NameMismatch::new(&other_exists.var, &self.var).into());
        }
        env.add_tyvar_super(other_exists.var, *self.sup_ty.clone());
        self.ty
            .clone()
            .normalize(env.clone())
            .check_subtype(&(*other_exists.ty), env)
    }
}

impl<Ty> Kindcheck<Ty> for ExistsBounded<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    fn check_kind(&self, mut env: Environment<Ty>) -> Result<Kind, CheckError<Ty>> {
        let sup_kind = self.sup_ty.check_kind(env.clone())?;
        env.add_tyvar_kind(self.var.clone(), sup_kind);
        self.ty.check_kind(env)
    }
}

impl<Ty> Normalize<Ty> for ExistsBounded<Ty>
where
    Ty: Type + Normalize<Ty>,
    Self: Into<Ty>,
{
    fn normalize(self, mut env: Environment<Ty>) -> Ty {
        env.add_tyvar_super(self.var.clone(), *self.sup_ty.clone());
        self.into()
    }
}
