use crate::{CheckResult, Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use syntax::{
    env::Environment,
    terms::{IsZero, Term},
    types::{Bool, TypeGroup},
};

impl<T> Typecheck for IsZero<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
    Bool<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<CheckResult<Self::Term, Self::Type>, Self::CheckError> {
        let inner_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        inner_ty.check_kind(env)?.into_star()?;
        inner_ty.into_nat()?;
        Ok(Bool::new().into())
    }
}
