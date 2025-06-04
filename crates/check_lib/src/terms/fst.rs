use crate::{CheckResult, Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use syntax::{
    env::Environment,
    terms::{Fst, Term},
    types::TypeGroup,
};

impl<T> Typecheck for Fst<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>
        + Normalize<<T as Typecheck>::Type>,
    <T as Typecheck>::CheckError: From<KindMismatch> + From<TypeMismatch>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<CheckResult<Self::Term, Self::Type>, Self::CheckError> {
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        term_ty.check_kind(env)?.into_star()?;
        let prod = term_ty.into_product()?;
        Ok(*prod.fst)
    }
}
