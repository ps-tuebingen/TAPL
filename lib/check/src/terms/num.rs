use crate::Typecheck;
use derivations::{Conclusion, Derivation, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{Num, Term},
    types::Nat,
};

impl<T> Typecheck for Num<T>
where
    T: Term + Typecheck<Term = T>,
    Nat<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        let conc = Conclusion::new(env.clone(), self.clone(), Nat::new());
        Ok(TypingDerivation::num(conc).into())
    }
}
