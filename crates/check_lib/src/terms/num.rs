use crate::{errors::CheckError, Typecheck};
use derivation::{Conclusion, Derivation};
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
    type Type = <T as Typecheck>::Type;
    type Term = T;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError<Self::Type>> {
        let conc = Conclusion::new(env.clone(), self.clone(), Nat::new());
        Ok(Derivation::num(conc))
    }
}
