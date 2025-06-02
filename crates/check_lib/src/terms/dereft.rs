use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use syntax::{
    terms::{Deref, Term},
    types::TypeGroup,
};

impl<T> Typecheck for Deref<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>
        + Kindcheck<
            <T as Typecheck>::Type,
            Env = <T as Typecheck>::Env,
            CheckError = <T as Typecheck>::CheckError,
        >,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
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
        let ref_ty = term_ty.into_ref()?;
        Ok(*ref_ty.ty)
    }
}
