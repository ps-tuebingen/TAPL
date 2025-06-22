use crate::{errors::CheckError, Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Head, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Head<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T>,
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError<Self::Type>> {
        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ty().normalize(env.clone());
        term_ty.check_kind(env.clone())?.into_star()?;
        let list_ty = term_ty.into_list()?;

        let conc = Conclusion::new(env.clone(), self.clone(), *list_ty.ty);
        let deriv = Derivation::head(conc, term_res);
        Ok(deriv)
    }
}
