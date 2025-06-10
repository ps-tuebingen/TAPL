use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Fst, Term},
    types::TypeGroup,
};

impl<T> Typecheck for Fst<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type: TypeGroup
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>
        + Normalize<<T as Typecheck>::Type>,
    <T as Typecheck>::CheckError: From<KindMismatch> + From<TypeMismatch>,
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
        let prod = term_ty.into_product()?;

        let conc = Conclusion::new(env, self.clone(), *prod.fst);
        let deriv = Derivation::fst(conc, term_res);
        Ok(deriv)
    }
}
