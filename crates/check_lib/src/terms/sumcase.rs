use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{SumCase, Term},
    types::TypeGroup,
};

impl<T> Typecheck for SumCase<T>
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
        let bound_res = self.bound_term.check(&mut env.clone())?;
        let bound_ty = bound_res.ty().normalize(&mut env.clone());
        bound_ty.check_kind(&mut env.clone())?.into_star()?;
        let bound_sum = bound_ty.into_sum()?;

        let mut left_env = env.clone();
        left_env.add_var(self.left_var.clone(), *bound_sum.left);
        let left_res = self.left_term.check(&mut left_env)?;
        let left_ty = left_res.ty().normalize(&mut left_env.clone());
        let left_knd = left_ty.check_kind(&mut left_env)?;

        env.add_var(self.right_var.clone(), *bound_sum.right);
        let right_res = self.right_term.check(&mut env.clone())?;
        let right_ty = right_res.ty().normalize(&mut env.clone());
        let right_knd = right_ty.check_kind(env)?;

        left_knd.check_equal(&right_knd)?;
        left_ty.check_equal(&right_ty)?;

        let conc = Conclusion::new(env.clone(), self.clone(), right_ty);
        let deriv = Derivation::sumcase(conc, bound_res, left_res, right_res);
        Ok(deriv)
    }
}
