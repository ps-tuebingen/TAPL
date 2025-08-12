use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Conclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{If, Term},
    types::TypeGroup,
};

impl<T> Typecheck for If<T>
where
    T: Term + Typecheck<Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    <T as Typecheck>::Type:
        TypeGroup + Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(&self, env: Environment<<T as Typecheck>::Type>) -> Result<Self::Deriv, CheckError> {
        let if_res = self.if_cond.check(env.clone())?;
        let if_ty = if_res.ty().normalize(env.clone());
        if_ty.check_kind(env.clone())?.into_star()?;
        if_ty.into_bool()?;

        let then_res = self.then_term.check(env.clone())?;
        let then_ty = then_res.ty().normalize(env.clone());
        let then_kind = then_ty.check_kind(env.clone())?;

        let else_res = self.else_term.check(env.clone())?;
        let else_ty = else_res.ty().normalize(env.clone());
        let else_kind = else_ty.check_kind(env.clone())?;

        then_kind.check_equal(&else_kind)?;
        then_ty.check_equal(&else_ty)?;

        let conc = Conclusion::new(env.clone(), self.clone(), then_ty);
        let deriv = TypingDerivation::ift(conc, if_res, then_res, else_res);
        Ok(deriv)
    }
}
