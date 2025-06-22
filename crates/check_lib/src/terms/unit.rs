use crate::{errors::CheckError, Typecheck};
use derivation::{Conclusion, TypingDerivation};
use syntax::{
    env::Environment,
    terms::{Term, Unit},
    types::Unit as UnitTy,
};

impl<T> Typecheck for Unit<T>
where
    T: Term + Typecheck<Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    UnitTy<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Self::Deriv, CheckError<Self::Type>> {
        let conc = Conclusion::new(env.clone(), self.clone(), UnitTy::new());
        let deriv = TypingDerivation::unit(conc);
        Ok(deriv.into())
    }
}
