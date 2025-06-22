use crate::{errors::CheckError, Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, TypingDerivation};
use syntax::{
    env::Environment,
    terms::{Right, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Right<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Self::Deriv, CheckError<Self::Type>> {
        let right_res = self.right_term.check(env.clone())?;
        let right_ty = right_res.ty().normalize(env.clone());
        let right_knd = right_ty.check_kind(env.clone())?;

        let sum_ty = self.ty.clone().normalize(env.clone()).into_sum()?;
        let sum_knd = sum_ty.check_kind(env.clone())?;

        right_knd.check_equal(&sum_knd)?;
        sum_ty.right.check_equal(&right_ty)?;

        let conc = Conclusion::new(env, self.clone(), self.ty.clone());
        let deriv = TypingDerivation::right(conc, right_res);

        Ok(deriv.into())
    }
}
