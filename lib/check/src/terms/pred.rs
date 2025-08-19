use crate::{Normalize, Typecheck};
use derivations::{Conclusion, Derivation, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{Pred, Term},
    types::{Nat, TypeGroup},
};

impl<T> Typecheck for Pred<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type: TypeGroup + Normalize<<T as Typecheck>::Type>,
    Nat<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        let inner_res = self.term.check(env.clone())?;
        let inner_ty = inner_res.ret_ty().normalize(env.clone());
        let nat = inner_ty.into_nat()?;

        let conc = Conclusion::new(env, self.clone(), nat);
        let deriv = TypingDerivation::pred(conc, inner_res);

        Ok(deriv.into())
    }
}
