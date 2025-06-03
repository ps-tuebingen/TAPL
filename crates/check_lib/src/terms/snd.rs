use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::TypeMismatch;
use syntax::{
    env::Environment,
    terms::{Snd, Term},
    types::TypeGroup,
};

impl<T> Typecheck for Snd<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
    <T as Typecheck>::CheckError: From<TypeMismatch>,
{
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Self::Type, Self::CheckError> {
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        term_ty.check_kind(env)?;
        let prod_ty = term_ty.into_product()?;
        Ok(*prod_ty.snd)
    }
}
