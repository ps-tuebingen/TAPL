use crate::{Kindcheck, Normalize, Typecheck};
use syntax::{
    terms::{Pair, Term},
    types::Product,
};

impl<T> Typecheck for Pair<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>
        + Kindcheck<
            <T as Typecheck>::Type,
            Env = <T as Typecheck>::Env,
            CheckError = <T as Typecheck>::CheckError,
        >,
    <T as Typecheck>::CheckError: From<syntax::errors::Error>,
    Product<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        let fst_ty = self
            .fst
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let snd_ty = self
            .snd
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let fst_knd = fst_ty.check_kind(&mut env.clone())?;
        let snd_knd = snd_ty.check_kind(env)?;
        fst_knd.check_equal(&snd_knd)?;
        Ok(Product::new(fst_ty, snd_ty).into())
    }
}
