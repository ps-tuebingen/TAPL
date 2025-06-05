use crate::Typecheck;
use common::errors::FreeVariable;
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Term, Variable},
};

impl<T> Typecheck for Variable<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::CheckError: From<FreeVariable>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let ty = env.get_var(&self.var)?;
        let conc = Conclusion::new(env.clone(), self.clone(), ty);
        let deriv = Derivation::var(conc);
        Ok(deriv)
    }
}
