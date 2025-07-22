use crate::{Normalize, Typecheck};
use derivation::{Conclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{False, Term},
    types::{Bool, Type},
};

impl<T> Typecheck for False<T>
where
    T: Term + Typecheck<Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    <T as Typecheck>::Type: Type + Normalize<<T as Typecheck>::Type>,
    Bool<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(&self, env: Environment<<T as Typecheck>::Type>) -> Result<Self::Deriv, CheckError> {
        let conc = Conclusion::new(env.clone(), self.clone(), Bool::new());
        Ok(TypingDerivation::fls(conc))
    }
}
