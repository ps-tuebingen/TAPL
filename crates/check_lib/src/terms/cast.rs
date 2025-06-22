use crate::{errors::CheckError, Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Cast, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Cast<T, Ty>
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
        let ty_norm = self.ty.clone().normalize(env.clone());
        let term_kind = term_ty.check_kind(env.clone())?;
        let ty_kind = ty_norm.check_kind(env.clone())?;
        term_kind.check_equal(&ty_kind)?;

        let conc = Conclusion::new(env, self.clone(), ty_norm.clone());
        let deriv = Derivation::cast(conc, term_res);
        Ok(deriv)
    }
}
