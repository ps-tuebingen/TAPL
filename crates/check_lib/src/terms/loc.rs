use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch, UndefinedLocation};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Loc, Term},
    types::Reference,
};

impl<T> Typecheck for Loc<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
    Reference<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch> + From<UndefinedLocation>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let loc_ty = env.get_loc(&self.loc)?.normalize(&mut env.clone());
        loc_ty.check_kind(env)?.into_star()?;

        let conc = Conclusion::new(env.clone(), self.clone(), Reference::new(loc_ty));
        let deriv = Derivation::loc(conc);
        Ok(deriv)
    }
}
