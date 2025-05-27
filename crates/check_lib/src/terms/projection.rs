use crate::{to_check_err, Kindcheck, Normalize, Typecheck};
use common::errors::{Error, ErrorKind};
use syntax::{
    terms::{Projection, Term},
    types::TypeGroup,
};

impl<T> Typecheck for Projection<T>
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
        let tup_ty = term_ty.into_tuple().map_err(to_check_err)?;
        tup_ty
            .tys
            .get(self.index)
            .ok_or(to_check_err(ErrorKind::Arity {
                found: tup_ty.tys.len(),
                expected: self.index,
            }))
            .cloned()
    }
}
