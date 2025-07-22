use crate::{Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, TypingDerivation};
use errors::IndexOutOfBounds;
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{Projection, Term},
    types::TypeGroup,
};

impl<T> Typecheck for Projection<T>
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
        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ty().normalize(env.clone());
        term_ty.check_kind(env.clone())?.into_star()?;
        let tup_ty = term_ty.into_tuple()?;
        let tup = tup_ty
            .tys
            .get(self.index)
            .ok_or(IndexOutOfBounds::new(self.index, tup_ty.tys.len()))
            .cloned()?;
        let conc = Conclusion::new(env, self.clone(), tup);
        let deriv = TypingDerivation::projection(conc, term_res);
        Ok(deriv)
    }
}
