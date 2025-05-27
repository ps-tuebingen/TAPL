use crate::{to_check_err, Kindcheck, Normalize, Typecheck};
use common::errors::Error;
use syntax::{
    terms::{Something, Term},
    types::{Optional, TypeGroup},
};

impl<T> Typecheck for Something<T>
where
    T: Term + Typecheck,
    Optional<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        term_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        Ok(Optional::new(term_ty.clone()).into())
    }
}
