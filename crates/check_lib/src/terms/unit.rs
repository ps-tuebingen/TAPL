use crate::Typecheck;
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Term, Unit},
    types::Unit as UnitTy,
};

impl<T> Typecheck for Unit<T>
where
    T: Term + Typecheck<Term = T>,
    UnitTy<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let conc = Conclusion::new(env.clone(), self.clone(), UnitTy::new());
        let deriv = Derivation::unit(conc);
        Ok(deriv)
    }
}
