use crate::{Normalize, Typecheck, errors::CheckError};
use derivation::{Conclusion, TypingDerivation};
use syntax::{
    env::Environment,
    terms::{Pred, Term},
    types::{Nat, TypeGroup},
};

impl<T> Typecheck for Pred<T>
where
    T: Term + Typecheck<Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    <T as Typecheck>::Type: TypeGroup + Normalize<<T as Typecheck>::Type>,
    Nat<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(&self, env: Environment<<T as Typecheck>::Type>) -> Result<Self::Deriv, CheckError> {
        let inner_res = self.term.check(env.clone())?;
        let inner_ty = inner_res.ty().normalize(env.clone());
        let nat = inner_ty.into_nat()?;

        let conc = Conclusion::new(env, self.clone(), nat);
        let deriv = TypingDerivation::pred(conc, inner_res);

        Ok(deriv)
    }
}
