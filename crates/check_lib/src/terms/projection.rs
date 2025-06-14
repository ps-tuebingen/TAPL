use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{IndexOutOfBounds, KindMismatch, TypeMismatch};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Projection, Term},
    types::TypeGroup,
};

impl<T> Typecheck for Projection<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch> + From<IndexOutOfBounds>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
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
        let deriv = Derivation::projection(conc, term_res);
        Ok(deriv)
    }
}
