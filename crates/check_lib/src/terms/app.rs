use crate::{to_check_err, Kindcheck, Normalize, Subtypecheck, Typecheck};
use common::errors::Error;
use syntax::{
    terms::{App, Term},
    types::{Fun, TypeGroup},
};

impl<T> Typecheck for App<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type>
        + Subtypecheck<<T as Typecheck>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let fun_ty = self
            .fun
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        fun_ty
            .check_kind(&mut env.clone())?
            .into_star()
            .map_err(to_check_err)?;
        let fun: Fun<<T as Typecheck>::Type> = fun_ty.into_fun().map_err(to_check_err)?;
        let arg_ty = self
            .arg
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        arg_ty
            .check_kind(&mut env.clone())?
            .into_star()
            .map_err(to_check_err)?;
        arg_ty.check_subtype(&(*fun.from), env)?;
        Ok(*fun.to)
    }
}
