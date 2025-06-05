use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Term, Try},
    types::TypeGroup,
};

impl<T> Typecheck for Try<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
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
        let term_res = self.term.check(&mut env.clone())?;
        let term_ty = term_res.ty().normalize(&mut env.clone());
        let term_knd = term_ty.check_kind(&mut env.clone())?;

        let handler_res = self.handler.check(&mut env.clone())?;
        let handler_ty = handler_res.ty().normalize(&mut env.clone());
        let handler_knd = handler_ty.check_kind(env)?;

        term_knd.check_equal(&handler_knd)?;
        term_ty.check_equal(&handler_ty)?;

        let conc = Conclusion::new(env.clone(), self.clone(), term_ty);
        let deriv = Derivation::tryt(conc, term_res, handler_res);
        Ok(deriv)
    }
}
