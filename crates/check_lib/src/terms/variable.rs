use crate::{errors::CheckError, Typecheck};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Term, Variable},
};

impl<T> Typecheck for Variable<T>
where
    T: Term + Typecheck<Term = T>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError<Self::Type>> {
        let ty = env.get_var(&self.var)?;
        let conc = Conclusion::new(env.clone(), self.clone(), ty);
        let deriv = Derivation::var(conc);
        Ok(deriv)
    }
}
