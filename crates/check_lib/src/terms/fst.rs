use crate::{to_check_err, Kindcheck, Normalize, Typecheck};
use common::errors::Error;
use syntax::{
    terms::{Fst, Term},
    types::TypeGroup,
};

impl<T> Typecheck for Fst<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type:
        TypeGroup + Kindcheck<<T as Typecheck>::Type> + Normalize<<T as Typecheck>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        term_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        let prod = term_ty.into_product().map_err(to_check_err)?;
        Ok(*prod.fst)
    }
}
