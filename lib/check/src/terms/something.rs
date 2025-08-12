use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Conclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{Something, Term},
    types::Optional,
};

impl<T> Typecheck for Something<T>
where
    T: Term + Typecheck<Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    Optional<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(&self, env: Environment<<T as Typecheck>::Type>) -> Result<Self::Deriv, CheckError> {
        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ty().normalize(env.clone());
        term_ty.check_kind(env.clone())?.into_star()?;
        let conc = Conclusion::new(env, self.clone(), Optional::new(term_ty.clone()));
        let deriv = TypingDerivation::something(conc, term_res);
        Ok(deriv)
    }
}
