use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Term, TyLambda},
    types::Forall,
};

impl<T> Typecheck for TyLambda<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
    Forall<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
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
        env.add_tyvar_kind(self.var.clone(), self.annot.clone());
        let term_res = self.term.check(&mut env.clone())?;
        let term_ty = term_res.ty().normalize(&mut env.clone());
        let term_knd = term_ty.check_kind(env)?;
        self.annot.check_equal(&term_knd)?;
        let ty = Forall::new(&self.var, self.annot.clone(), term_ty);
        let conc = Conclusion::new(env.clone(), self.clone(), ty);
        let deriv = Derivation::tylambda(conc, term_res);
        Ok(deriv)
    }
}
