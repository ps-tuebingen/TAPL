use crate::Typecheck;
use syntax::{
    terms::{Num, Term},
    types::Nat,
};

impl<T> Typecheck for Num<T>
where
    T: Term + Typecheck,
    Nat<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;
    type Env = <T as Typecheck>::Env;

    fn check(&self, _: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        Ok(Nat::new().into())
    }
}
