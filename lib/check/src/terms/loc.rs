use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Conclusion, Derivation, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{Loc, Term},
    types::Reference,
};

impl<T> Typecheck for Loc<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
    Reference<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        let loc_ty = env.get_loc(&self.loc)?.normalize(env.clone());
        loc_ty.check_kind(env.clone())?.into_star()?;

        let conc = Conclusion::new(env, self.clone(), Reference::new(loc_ty));
        let deriv = TypingDerivation::loc(conc);
        Ok(deriv.into())
    }
}
