use crate::{errors::IndexOutOfBounds, Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use syntax::{
    terms::{Projection, Term},
    types::TypeGroup,
};

impl<T> Typecheck for Projection<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>
        + Kindcheck<
            <T as Typecheck>::Type,
            Env = <T as Typecheck>::Env,
            CheckError = <T as Typecheck>::CheckError,
        >,
    <T as Typecheck>::CheckError: From<IndexOutOfBounds>,
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
        let tup_ty = term_ty.into_tuple()?;
        tup_ty
            .tys
            .get(self.index)
            .ok_or(IndexOutOfBounds::new(self.index, tup_ty.tys.len()).into())
            .cloned()
    }
}
