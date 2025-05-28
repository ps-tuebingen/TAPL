use crate::{to_check_err, Kindcheck, Normalize, Typecheck};
use common::errors::Error;
use syntax::{
    terms::{Term, TryWithVal},
    types::{Fun, TypeGroup},
};

impl<T> Typecheck for TryWithVal<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>
        + Kindcheck<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let t_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let t_knd = t_ty.check_kind(&mut env.clone())?;

        let handler_ty = self
            .handler
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let handler_knd = handler_ty.check_kind(env)?;
        let fun: Fun<<T as Typecheck>::Type> = handler_ty.into_fun().map_err(to_check_err)?;

        t_knd.check_equal(&handler_knd).map_err(to_check_err)?;
        fun.to.check_equal(&t_ty).map_err(to_check_err)?;
        Ok(t_ty)
    }
}
