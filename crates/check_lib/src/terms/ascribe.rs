use crate::{errors::CheckError, Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Ascribe, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Ascribe<T, Ty>
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
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError<Ty>> {
        let t_res = self.term.check(env.clone())?;
        let t_ty = t_res.ty().normalize(env.clone());
        let asc_norm = self.ty.clone().normalize(env.clone());
        let t_kind = t_ty.check_kind(env.clone())?;
        let ty_kind = self.ty.check_kind(env.clone())?;
        t_kind.check_equal(&ty_kind)?;
        asc_norm.check_equal(&t_ty)?;

        let conc = Conclusion::new(env, self.clone(), self.ty.clone());
        let deriv = Derivation::ascribe(conc, t_res);
        Ok(deriv)
    }
}
