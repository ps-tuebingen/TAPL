use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::KindMismatch;
use syntax::{
    terms::{Something, Term},
    types::Optional,
};

impl<T> Typecheck for Something<T>
where
    T: Term + Typecheck,
    Optional<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>
        + Kindcheck<
            <T as Typecheck>::Type,
            Env = <T as Typecheck>::Env,
            CheckError = <T as Typecheck>::CheckError,
        >,
    <T as Typecheck>::CheckError: From<KindMismatch>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        term_ty.check_kind(env)?.into_star()?;
        Ok(Optional::new(term_ty.clone()).into())
    }
}
