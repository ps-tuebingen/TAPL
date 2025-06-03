use crate::{Kindcheck, Normalize, Subtypecheck, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use syntax::{
    env::Environment,
    terms::{App, Term},
    types::{Fun, TypeGroup},
};

impl<T> Typecheck for App<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>
        + Subtypecheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
{
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Self::Type, Self::CheckError> {
        let fun_ty = self
            .fun
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        fun_ty.check_kind(&mut env.clone())?.into_star()?;
        let fun: Fun<<T as Typecheck>::Type> = fun_ty.into_fun()?;
        let arg_ty = self
            .arg
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        arg_ty.check_kind(&mut env.clone())?.into_star()?;
        arg_ty.check_subtype(&(*fun.from), env)?;
        Ok(*fun.to)
    }
}
