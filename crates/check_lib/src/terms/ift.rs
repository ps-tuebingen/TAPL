use crate::{to_check_err, Kindcheck, Normalize, Typecheck};
use common::errors::Error;
use syntax::{
    terms::{If, Term},
    types::TypeGroup,
};

impl<T> Typecheck for If<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type:
        TypeGroup + Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let if_ty = self
            .if_cond
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        if_ty
            .check_kind(&mut env.clone())?
            .into_star()
            .map_err(to_check_err)?;
        if_ty.into_bool().map_err(to_check_err)?;

        let then_ty = self
            .then_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let then_kind = then_ty.check_kind(&mut env.clone())?;

        let else_ty = self
            .else_term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let else_kind = else_ty.check_kind(env)?;

        then_kind.check_equal(&else_kind).map_err(to_check_err)?;
        then_ty.check_equal(&else_ty).map_err(to_check_err)?;
        Ok(then_ty)
    }
}
