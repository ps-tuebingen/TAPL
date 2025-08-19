use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Conclusion, Derivation, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{Term, Try},
    types::TypeGroup,
};

impl<T> Typecheck for Try<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type:
        TypeGroup + Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ret_ty().normalize(env.clone());
        let term_knd = term_ty.check_kind(env.clone())?;

        let handler_res = self.handler.check(env.clone())?;
        let handler_ty = handler_res.ret_ty().normalize(env.clone());
        let handler_knd = handler_ty.check_kind(env.clone())?;

        term_knd.check_equal(&handler_knd)?;
        term_ty.check_equal(&handler_ty)?;

        let conc = Conclusion::new(env, self.clone(), term_ty);
        let deriv = TypingDerivation::tryt(conc, term_res, handler_res);
        Ok(deriv.into())
    }
}
