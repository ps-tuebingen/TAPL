use crate::{Kindcheck, Normalize, Typecheck, errors::CheckError};
use derivation::{Conclusion, TypingDerivation};
use syntax::{
    env::Environment,
    terms::{SumCase, Term},
    types::TypeGroup,
};

impl<T> Typecheck for SumCase<T>
where
    T: Term + Typecheck<Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    <T as Typecheck>::Type:
        TypeGroup + Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(
        &self,
        mut env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Self::Deriv, CheckError> {
        let bound_res = self.bound_term.check(env.clone())?;
        let bound_ty = bound_res.ty().normalize(env.clone());
        bound_ty.check_kind(env.clone())?.into_star()?;
        let bound_sum = bound_ty.into_sum()?;

        let mut left_env = env.clone();
        left_env.add_var(self.left_var.clone(), *bound_sum.left);
        let left_res = self.left_term.check(left_env.clone())?;
        let left_ty = left_res.ty().normalize(left_env.clone());
        let left_knd = left_ty.check_kind(left_env)?;

        env.add_var(self.right_var.clone(), *bound_sum.right);
        let right_res = self.right_term.check(env.clone())?;
        let right_ty = right_res.ty().normalize(env.clone());
        let right_knd = right_ty.check_kind(env.clone())?;

        left_knd.check_equal(&right_knd)?;
        left_ty.check_equal(&right_ty)?;

        let conc = Conclusion::new(env.clone(), self.clone(), right_ty);
        let deriv = TypingDerivation::sumcase(conc, bound_res, left_res, right_res);
        Ok(deriv)
    }
}
