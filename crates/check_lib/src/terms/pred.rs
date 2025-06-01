use crate::{Normalize, Typecheck};
use syntax::{
    terms::{Pred, Term},
    types::{Nat, TypeGroup},
};

impl<T> Typecheck for Pred<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type:
        TypeGroup + Normalize<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>,
    Nat<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    <T as Typecheck>::CheckError: From<syntax::errors::Error>,
{
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        let inner_ty = self.term.check(&mut env.clone())?.normalize(env);
        let nat = inner_ty.into_nat()?;
        Ok(nat.into())
    }
}
