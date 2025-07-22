use crate::{Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{Left, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Left<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(&self, env: Environment<<T as Typecheck>::Type>) -> Result<Self::Deriv, CheckError> {
        let left_res = self.left_term.check(env.clone())?;
        let left_ty = left_res.ty().normalize(env.clone());
        let left_knd = left_ty.check_kind(env.clone())?;
        let ty_norm = self.ty.clone().normalize(env.clone());
        let sum_ty = ty_norm.into_sum()?;
        let sum_kind = sum_ty.check_kind(env.clone())?;
        left_knd.check_equal(&sum_kind)?;
        sum_ty.left.check_equal(&left_ty)?;

        let conc = Conclusion::new(env, self.clone(), self.ty.clone());
        let deriv = TypingDerivation::left(conc, left_res);
        Ok(deriv)
    }
}
