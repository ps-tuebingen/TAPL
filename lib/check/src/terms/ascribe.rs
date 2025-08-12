use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Conclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{Ascribe, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Ascribe<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T, Deriv = TypingDerivation<T, Ty>>,
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;
    type Term = T;

    fn check(&self, env: Environment<<T as Typecheck>::Type>) -> Result<Self::Deriv, CheckError> {
        let t_res = self.term.check(env.clone())?;
        let t_ty = t_res.ty().normalize(env.clone());
        let asc_norm = self.ty.clone().normalize(env.clone());
        let t_kind = t_ty.check_kind(env.clone())?;
        let ty_kind = self.ty.check_kind(env.clone())?;
        t_kind.check_equal(&ty_kind)?;
        asc_norm.check_equal(&t_ty)?;

        let conc = Conclusion::new(env, self.clone(), self.ty.clone());
        let deriv = TypingDerivation::ascribe(conc, t_res);
        Ok(deriv)
    }
}
