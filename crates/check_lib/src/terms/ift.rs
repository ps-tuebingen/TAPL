use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{If, Term},
    types::TypeGroup,
};

impl<T> Typecheck for If<T>
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
        let if_res = self.if_cond.check(&mut env.clone())?;
        let if_ty = if_res.ty().normalize(&mut env.clone());
        if_ty.check_kind(&mut env.clone())?.into_star()?;
        if_ty.into_bool()?;

        let then_res = self.then_term.check(&mut env.clone())?;
        let then_ty = then_res.ty().normalize(&mut env.clone());
        let then_kind = then_ty.check_kind(&mut env.clone())?;

        let else_res = self.else_term.check(&mut env.clone())?;
        let else_ty = else_res.ty().normalize(&mut env.clone());
        let else_kind = else_ty.check_kind(&mut env.clone())?;

        then_kind.check_equal(&else_kind)?;
        then_ty.check_equal(&else_ty)?;

        let conc = Conclusion::new(env.clone(), self.clone(), then_ty);
        let deriv = Derivation::ift(conc, if_res, then_res, else_res);
        Ok(deriv)
    }
}
