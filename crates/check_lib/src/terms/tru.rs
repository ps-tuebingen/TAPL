use crate::Typecheck;
use syntax::{
    terms::{Term, True},
    types::Bool,
};

impl<T> Typecheck for True<T>
where
    T: Term + Typecheck,
    Bool<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;
    type Env = <T as Typecheck>::Env;

    fn check(&self, _: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        Ok(Bool::new().into())
    }
}
