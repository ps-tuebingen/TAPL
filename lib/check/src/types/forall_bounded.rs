use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
use errors::NameMismatch;
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{ForallBounded, Top, Type, TypeGroup},
};

impl<Ty> Subtypecheck for ForallBounded<Ty>
where
    Ty: TypeGroup + Subtypecheck<Type = Ty> + Normalize<Ty>,
    ForallBounded<Ty>: Into<Ty>,
    Top<Ty>: Into<Ty>,
{
    type Type = Ty;
    type Term = <Ty as Subtypecheck>::Term;
    fn check_subtype(
        &self,
        sup: &Ty,
        env: Environment<Ty>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        if let Ok(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind).into());
        }
        let other_forall = sup.clone().into_forall_bounded()?;
        let sup_norm = other_forall.sup_ty.normalize(env.clone());
        let self_norm = self.sup_ty.clone().normalize(env.clone());
        let sup_res = sup_norm.check_subtype(&self_norm, env.clone())?;
        if self.var != other_forall.var {
            return Err(NameMismatch::new(&other_forall.var, &self.var).into());
        }
        let ty_norm = self.ty.clone().normalize(env.clone());
        let inner_res = ty_norm.check_subtype(&(*other_forall.ty), env.clone())?;
        Ok(
            SubtypeDerivation::forall_bounded(env, self.clone(), sup.clone(), sup_res, inner_res)
                .into(),
        )
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
