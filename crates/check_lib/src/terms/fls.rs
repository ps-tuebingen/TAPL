use crate::{Normalize, Typecheck};
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
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        _: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Self::Type, Self::CheckError> {
        Ok(Bool::new().into())
    }
}
