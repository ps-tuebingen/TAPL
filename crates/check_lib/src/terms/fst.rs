use crate::{Kindcheck, Normalize, Typecheck};
use syntax::{
    terms::{Fst, Term},
    types::TypeGroup,
};

impl<T> Typecheck for Fst<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup
        + Kindcheck<
            <T as Typecheck>::Type,
            Env = <T as Typecheck>::Env,
            CheckError = <T as Typecheck>::CheckError,
        > + Normalize<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>,
    <T as Typecheck>::CheckError: From<syntax::errors::Error>,
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
        let prod = term_ty.into_product()?;
        Ok(*prod.fst)
    }
}
