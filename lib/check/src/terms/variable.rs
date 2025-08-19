use crate::Typecheck;
use derivations::{Conclusion, Derivation, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{Term, Variable},
};

impl<T> Typecheck for Variable<T>
where
    T: Term + Typecheck<Term = T>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        let ty = env.get_var(&self.var)?;
        let conc = Conclusion::new(env.clone(), self.clone(), ty);
        let deriv = TypingDerivation::var(conc);
        Ok(deriv.into())
    }
}
