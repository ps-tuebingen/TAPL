use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{Right, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Right<T, Ty>
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
        let right_res = self.right_term.check(env.clone())?;
        let right_ty = right_res.ret_ty().normalize(env.clone());
        let right_knd = right_ty.check_kind(env.clone())?;

        let sum_ty = self.ty.clone().normalize(env.clone()).into_sum()?;
        let sum_knd = sum_ty.check_kind(env.clone())?;

        right_knd.check_equal(&sum_knd)?;
        sum_ty.right.check_equal(&right_ty)?;

        let conc = TypingConclusion::new(env, self.clone(), self.ty.clone());
        let deriv = TypingDerivation::right(conc, right_res);

        Ok(deriv.into())
    }
}
