use crate::{Typecheck, errors::CheckError};
use derivation::{Conclusion, TypingDerivation};
use syntax::{
    env::Environment,
    terms::{Num, Term},
    types::Nat,
};

impl<T> Typecheck for Num<T>
where
    T: Term + Typecheck<Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    Nat<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(&self, env: Environment<<T as Typecheck>::Type>) -> Result<Self::Deriv, CheckError> {
        let conc = Conclusion::new(env.clone(), self.clone(), Nat::new());
        Ok(TypingDerivation::num(conc))
    }
}
