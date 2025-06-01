use crate::{Normalize, Typecheck};
use syntax::{
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
    type Env = <T as Typecheck>::Env;

    fn check(&self, _: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        Ok(Bool::new().into())
    }
}
