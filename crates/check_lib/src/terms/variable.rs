use crate::{ Typecheck};
use derivation::{Derivation,Conclusion};
use common::errors::FreeVariable;
use syntax::{
    env::Environment,
    terms::{Term, Variable},
};

impl<T> Typecheck for Variable<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::CheckError: From<FreeVariable>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<<Self::Term, Self::Type>, Self::CheckError> {
        env.get_var(&self.var).map_err(|err| err.into())
    }
}
