use crate::{CheckResult, Normalize, Typecheck};
use syntax::{
    env::Environment,
    terms::{False, Term},
    types::{Bool, Type},
};

impl<T> Typecheck for False<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: Type + Normalize<<T as Typecheck>::Type>,
    Bool<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        _: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<CheckResult<Self::Term, Self::Type>, Self::CheckError> {
        Ok(Bool::new().into())
    }
}
