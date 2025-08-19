use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{Left, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Left<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T>,
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        let left_res = self.left_term.check(env.clone())?;
        let left_ty = left_res.ret_ty().normalize(env.clone());
        let left_knd = left_ty.check_kind(env.clone())?;
        let ty_norm = self.ty.clone().normalize(env.clone());
        let sum_ty = ty_norm.into_sum()?;
        let sum_kind = sum_ty.check_kind(env.clone())?;
        left_knd.check_equal(&sum_kind)?;
        sum_ty.left.check_equal(&left_ty)?;

        let conc = TypingConclusion::new(env, self.clone(), self.ty.clone());
        let deriv = TypingDerivation::left(conc, left_res);
        Ok(deriv.into())
    }
}
