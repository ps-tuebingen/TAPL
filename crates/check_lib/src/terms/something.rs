use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::KindMismatch;
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Something, Term},
    types::Optional,
};

impl<T> Typecheck for Something<T>
where
    T: Term + Typecheck<Term = T>,
    Optional<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
    <T as Typecheck>::CheckError: From<KindMismatch>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ty().normalize(env.clone());
        term_ty.check_kind(env.clone())?.into_star()?;
        let conc = Conclusion::new(env, self.clone(), Optional::new(term_ty.clone()));
        let deriv = Derivation::something(conc, term_res);
        Ok(deriv)
    }
}
