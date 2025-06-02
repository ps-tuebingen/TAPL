use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use syntax::{
    terms::{IsZero, Term},
    types::{Bool, TypeGroup},
};

impl<T> Typecheck for IsZero<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>
        + Kindcheck<
            <T as Typecheck>::Type,
            Env = <T as Typecheck>::Env,
            CheckError = <T as Typecheck>::CheckError,
        >,
    Bool<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
{
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        let inner_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        inner_ty.check_kind(env)?.into_star()?;
        inner_ty.into_nat()?;
        Ok(Bool::new().into())
    }
}
