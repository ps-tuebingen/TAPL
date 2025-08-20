use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
use errors::{NameMismatch, check_error::CheckError};
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{ExistsBounded, Top, Type, TypeGroup},
};

impl<Ty> Subtypecheck for ExistsBounded<Ty>
where
    Ty: TypeGroup + Subtypecheck<Type = Ty> + Normalize<Ty>,
    Top<Ty>: Into<Ty>,
    ExistsBounded<Ty>: Into<Ty>,
{
    type Type = Ty;
    type Term = <Ty as Subtypecheck>::Term;
    fn check_subtype(
        &self,
        sup: &Ty,
        mut env: Environment<Ty>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        if let Ok(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind).into());
        }

        let sup_norm = sup.clone().normalize(env.clone());
        let self_norm = self.sup_ty.clone().normalize(env.clone());
        let other_exists = sup_norm.into_exists_bounded()?;
        other_exists.sup_ty.check_equal(&self_norm)?;
        if self.var != other_exists.var {
            return Err(NameMismatch::new(&other_exists.var, &self.var).into());
        }
        let old_env = env.clone();
        env.add_tyvar_super(other_exists.var, *self.sup_ty.clone());
        let inner_res = self
            .ty
            .clone()
            .normalize(env.clone())
            .check_subtype(&(*other_exists.ty), env)?;
        Ok(SubtypeDerivation::exists_bounded(old_env, self.clone(), sup.clone(), inner_res).into())
    }
}

impl<Ty> Kindcheck<Ty> for ExistsBounded<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    fn check_kind(&self, mut env: Environment<Ty>) -> Result<Kind, CheckError> {
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
