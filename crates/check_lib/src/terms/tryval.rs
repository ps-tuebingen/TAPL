use crate::{errors::CheckError, Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Term, TryWithVal},
    types::{Fun, TypeGroup},
};

impl<T> Typecheck for TryWithVal<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type:
        TypeGroup + Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError<Self::Type>> {
        let t_res = self.term.check(env.clone())?;
        let t_ty = t_res.ty().normalize(env.clone());
        let t_knd = t_ty.check_kind(env.clone())?;

        let handler_res = self.handler.check(env.clone())?;
        let handler_ty = handler_res.ty().normalize(env.clone());
        let handler_knd = handler_ty.check_kind(env.clone())?;
        let fun: Fun<<T as Typecheck>::Type> = handler_ty.into_fun()?;

        t_knd.check_equal(&handler_knd)?;
        fun.to.check_equal(&t_ty)?;

        let conc = Conclusion::new(env, self.clone(), t_ty);
        let deriv = Derivation::try_val(conc, t_res, handler_res);
        Ok(deriv)
    }
}
