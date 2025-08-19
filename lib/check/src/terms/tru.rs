use crate::Typecheck;
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
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
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        let conc = TypingConclusion::new(env.clone(), self.clone(), Bool::new());
        let deriv = TypingDerivation::tru(conc);
        Ok(deriv.into())
    }
}
