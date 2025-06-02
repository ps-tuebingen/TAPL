use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use syntax::{
    terms::{Left, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Left<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: TypeGroup
        + Normalize<Ty, Env = <T as Typecheck>::Env>
        + Kindcheck<Ty, Env = <T as Typecheck>::Env, CheckError = <T as Typecheck>::CheckError>,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        let left_ty = self
            .left_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let left_knd = left_ty.check_kind(&mut env.clone())?;
        let ty_norm = self.ty.clone().normalize(&mut env.clone());
        let sum_ty = ty_norm.into_sum()?;
        let sum_kind = sum_ty.check_kind(env)?;
        left_knd.check_equal(&sum_kind)?;
        sum_ty.left.check_equal(&left_ty)?;
        Ok(self.ty.clone())
    }
}
