use crate::{ Typecheck};
use derivation::{Derivation,Conclusion};
use syntax::{
    env::Environment,
    terms::{Term, True},
    types::Bool,
};

impl<T> Typecheck for True<T>
where
    T: Term + Typecheck,
    Bool<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        _: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<<Self::Term, Self::Type>, Self::CheckError> {
        Ok(Bool::new().into())
    }
}
