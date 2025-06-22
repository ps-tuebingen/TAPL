use crate::{Kindcheck, Normalize, Subtypecheck, errors::CheckError};
use common::errors::NameMismatch;
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{ForallBounded, Type, TypeGroup},
};

impl<Ty> Subtypecheck<Ty> for ForallBounded<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty> + Normalize<Ty>,
{
    fn check_subtype(&self, sup: &Ty, env: Environment<Ty>) -> Result<(), CheckError> {
        let other_forall = sup.clone().into_forall_bounded()?;
        let sup_norm = other_forall.sup_ty.normalize(env.clone());
        let self_norm = self.sup_ty.clone().normalize(env.clone());
        sup_norm.check_equal(&self_norm)?;
        if self.var != other_forall.var {
            return Err(NameMismatch::new(&other_forall.var, &self.var).into());
        }
        let ty_norm = self.ty.clone().normalize(env.clone());
        ty_norm.check_subtype(&(*other_forall.ty), env)
    }
}

impl<Ty> Kindcheck<Ty> for ForallBounded<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    fn check_kind(&self, mut env: Environment<Ty>) -> Result<Kind, CheckError> {
        let sup_kind = self.sup_ty.check_kind(env.clone())?;
        env.add_tyvar_kind(self.var.clone(), sup_kind);
        self.ty.check_kind(env)
    }
}

impl<Ty> Normalize<Ty> for ForallBounded<Ty>
where
    Ty: Type + Normalize<Ty>,
    Self: Into<Ty>,
{
    fn normalize(self, mut env: Environment<Ty>) -> Ty {
        env.add_tyvar_super(self.var.clone(), *self.ty.clone());
        let ty_norm = self.ty.normalize(env);
        ForallBounded {
            var: self.var,
            sup_ty: self.sup_ty,
            ty: Box::new(ty_norm),
        }
        .into()
    }
}
