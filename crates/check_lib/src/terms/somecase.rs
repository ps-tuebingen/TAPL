use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use syntax::{
    env::Environment,
    terms::{SomeCase, Term},
    types::TypeGroup,
};

impl<T> Typecheck for SomeCase<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
{
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Self::Type, Self::CheckError> {
        let bound_ty = self
            .bound_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        bound_ty.check_kind(&mut env.clone())?.into_star()?;

        let option = bound_ty.into_optional()?;
        let mut some_env = env.clone();
        some_env.add_var(self.some_var.clone(), *option.ty);
        let some_ty = self
            .some_term
            .check(&mut some_env.clone())?
            .normalize(&mut some_env.clone());
        let some_knd = some_ty.check_kind(&mut some_env)?;

        let none_ty = self
            .none_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let none_knd = none_ty.check_kind(env)?;

        some_knd.check_equal(&none_knd)?;
        some_ty.check_equal(&none_ty)?;
        Ok(some_ty)
    }
}
