use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use syntax::{
    terms::{Term, TryWithVal},
    types::{Fun, TypeGroup},
};

impl<T> Typecheck for TryWithVal<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>
        + Kindcheck<
            <T as Typecheck>::Type,
            Env = <T as Typecheck>::Env,
            CheckError = <T as Typecheck>::CheckError,
        >,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
{
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
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
        let fun: Fun<<T as Typecheck>::Type> = handler_ty.into_fun()?;

        t_knd.check_equal(&handler_knd)?;
        fun.to.check_equal(&t_ty)?;
        Ok(t_ty)
    }
}
