use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{IsZero, Term},
    types::{Bool, TypeGroup},
};

impl<T> Typecheck for IsZero<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
    Bool<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
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
        inner_ty.into_nat()?;

        let conc = Conclusion::new(env.clone(), self.clone(), Bool::new());
        let deriv = Derivation::iszero(conc, inner_res);
        Ok(deriv)
    }
}
