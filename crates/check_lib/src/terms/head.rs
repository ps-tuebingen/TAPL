use crate::{Kindcheck, Normalize, Typecheck};
use syntax::{
    terms::{Head, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Head<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: TypeGroup
        + Normalize<Ty, Env = <T as Typecheck>::Env>
        + Kindcheck<Ty, Env = <T as Typecheck>::Env, CheckError = <T as Typecheck>::CheckError>,
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
        let list_ty = term_ty.into_list()?;
        Ok(*list_ty.ty)
    }
}
