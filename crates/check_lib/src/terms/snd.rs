use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::TypeMismatch;
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Snd, Term},
    types::TypeGroup,
};

impl<T> Typecheck for Snd<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
    <T as Typecheck>::CheckError: From<TypeMismatch>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let term_res = self.term.check(&mut env.clone())?;
        let term_ty = term_res.ty().normalize(&mut env.clone());
        term_ty.check_kind(env)?;
        let prod_ty = term_ty.into_product()?;

        let conc = Conclusion::new(env.clone(), self.clone(), *prod_ty.snd);
        let deriv = Derivation::snd(conc, term_res);
        Ok(deriv)
    }
}
