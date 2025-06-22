use crate::{errors::CheckError, Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, TypingDerivation};
use syntax::{
    env::Environment,
    subst::SubstType,
    terms::{Term, Unfold},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Unfold<T, Ty>
where
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty> + SubstType<Ty, Target = Ty>,
    T: Term + Typecheck<Type = Ty, Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(&self, env: Environment<<T as Typecheck>::Type>) -> Result<Self::Deriv, CheckError> {
        let ty_norm = self.ty.clone().normalize(env.clone());
        let ty_kind = ty_norm.check_kind(env.clone())?;

        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ty().normalize(env.clone());
        let term_knd = term_ty.check_kind(env.clone())?;
        term_knd.check_equal(&ty_kind)?;

        ty_norm.check_equal(&term_ty)?;
        let mu_ty = term_ty.clone().into_mu()?;
        let ty = mu_ty.ty.subst_type(&mu_ty.var, &term_ty);
        let conc = Conclusion::new(env.clone(), self.clone(), ty);
        let deriv = TypingDerivation::unfold(conc, term_res);
        Ok(deriv.into())
    }
}
