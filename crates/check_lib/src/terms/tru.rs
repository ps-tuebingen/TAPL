use crate::Typecheck;
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Term, True},
    types::Bool,
};

impl<T> Typecheck for True<T>
where
    T: Term + Typecheck<Term = T>,
    Bool<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let conc = Conclusion::new(env.clone(), self.clone(), Bool::new());
        let deriv = Derivation::tru(conc);
        Ok(deriv)
    }
}
