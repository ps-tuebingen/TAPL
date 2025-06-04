use crate::{CheckResult, Typecheck};
use syntax::{
    env::Environment,
    terms::{Num, Term},
    types::Nat,
};

impl<T> Typecheck for Num<T>
where
    T: Term + Typecheck,
    Nat<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        _: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<CheckResult<Self::Term, Self::Type>, Self::CheckError> {
        Ok(Nat::new().into())
    }
}
