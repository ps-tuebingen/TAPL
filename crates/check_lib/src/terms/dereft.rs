use crate::{to_check_err, Kindcheck, Normalize, Typecheck};
use common::errors::Error;
use syntax::{
    terms::{Deref, Term},
    types::TypeGroup,
};

impl<T> Typecheck for Deref<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type:
        TypeGroup + Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        term_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        let ref_ty = term_ty.into_ref().map_err(to_check_err)?;
        Ok(*ref_ty.ty)
    }
}
