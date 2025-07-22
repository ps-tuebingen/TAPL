use crate::Typecheck;
use derivation::{Conclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{Term, Variable},
};

impl<T> Typecheck for Variable<T>
where
    T: Term + Typecheck<Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(&self, env: Environment<<T as Typecheck>::Type>) -> Result<Self::Deriv, CheckError> {
        let ty = env.get_var(&self.var)?;
        let conc = Conclusion::new(env.clone(), self.clone(), ty);
        let deriv = TypingDerivation::var(conc);
        Ok(deriv)
    }
}
