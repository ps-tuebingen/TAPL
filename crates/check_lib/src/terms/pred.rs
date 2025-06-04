use crate::{Normalize, Typecheck};
use common::errors::TypeMismatch;
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Pred, Term},
    types::{Nat, TypeGroup},
};

impl<T> Typecheck for Pred<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup + Normalize<<T as Typecheck>::Type>,
    Nat<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    <T as Typecheck>::CheckError: From<TypeMismatch>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let inner_res = self.term.check(&mut env.clone())?;
        let inner_ty = inner_res.ty().normalize(env);
        let nat = inner_ty.into_nat()?;

        let conc = Conclusion::new(env.clone(), self.clone(), nat);
        let deriv = Derivation::pred(conc, inner_res);

        Ok(deriv)
    }
}
