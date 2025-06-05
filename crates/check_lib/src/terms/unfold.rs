use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    subst::SubstType,
    terms::{Term, Unfold},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Unfold<T, Ty>
where
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty> + SubstType<Ty, Target = Ty>,
    <T as Typecheck>::CheckError:
        From<TypeMismatch> + From<KindMismatch> + From<<Ty as Kindcheck<Ty>>::CheckError>,
    T: Term + Typecheck<Type = Ty, Term = T>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let ty_norm = self.ty.clone().normalize(&mut env.clone());
        let ty_kind = ty_norm.check_kind(&mut env.clone())?;

        let term_res = self.term.check(&mut env.clone())?;
        let term_ty = term_res.ty().normalize(&mut env.clone());
        let term_knd = term_ty.check_kind(env)?;
        term_knd.check_equal(&ty_kind)?;

        ty_norm.check_equal(&term_ty)?;
        let mu_ty = term_ty.clone().into_mu()?;
        let ty = mu_ty.ty.subst_type(&mu_ty.var, &term_ty);
        let conc = Conclusion::new(env.clone(), self.clone(), ty);
        let deriv = Derivation::unfold(conc, term_res);
        Ok(deriv)
    }
}
