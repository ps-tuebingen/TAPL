use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Conclusion, Derivation, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{Deref, Term},
    types::TypeGroup,
};

impl<T> Typecheck for Deref<T>
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
        term_ty.check_kind(env.clone())?.into_star()?;
        let ref_ty = term_ty.into_ref()?;

        let conc = Conclusion::new(env, self.clone(), *ref_ty.ty.clone());
        let deriv = TypingDerivation::deref(conc, term_res);

        Ok(deriv.into())
    }
}
