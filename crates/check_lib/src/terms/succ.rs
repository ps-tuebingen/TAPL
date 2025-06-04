use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Succ, Term},
    types::{Nat, TypeGroup},
};

impl<T> Typecheck for Succ<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
    Nat<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let inner_res = self.term.check(&mut env.clone())?;
        let inner_ty = inner_res.ty().normalize(&mut env.clone());
        inner_ty.check_kind(env)?.into_star()?;
        let nat = inner_ty.into_nat()?;

        let conc = Conclusion::new(env.clone(), self.clone(), nat);
        let deriv = Derivation::succ(conc, inner_res);
        Ok(deriv)
    }
}
