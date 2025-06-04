use crate::{CheckResult, Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use syntax::{
    env::Environment,
    terms::{If, Term},
    types::TypeGroup,
};

impl<T> Typecheck for If<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<CheckResult<Self::Term, Self::Type>, Self::CheckError> {
        let if_ty = self
            .if_cond
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        if_ty.check_kind(&mut env.clone())?.into_star()?;
        if_ty.into_bool()?;

        let then_ty = self
            .then_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let then_kind = then_ty.check_kind(&mut env.clone())?;

        let else_ty = self
            .else_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let else_kind = else_ty.check_kind(env)?;

        then_kind.check_equal(&else_kind)?;
        then_ty.check_equal(&else_ty)?;
        Ok(then_ty)
    }
}
