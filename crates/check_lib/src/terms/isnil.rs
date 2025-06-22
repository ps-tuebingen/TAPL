use crate::{errors::CheckError, Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{IsNil, Term},
    types::{Bool, List, TypeGroup},
};

impl<T, Ty> Typecheck for IsNil<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T>,
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty>,
    List<Ty>: Into<Ty>,
    Bool<Ty>: Into<Ty>,
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
        term_ty.into_list()?;

        let conc = Conclusion::new(env, self.clone(), Bool::new());
        let deriv = Derivation::isnil(conc, term_res);
        Ok(deriv)
    }
}
