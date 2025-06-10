use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch, UndefinedLabel};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Term, Variant},
    types::{TypeGroup, Variant as VariantTy},
};

impl<T, Ty> Typecheck for Variant<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T>,
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty>,
    VariantTy<Ty>: Into<Ty>,
    <T as Typecheck>::CheckError: From<UndefinedLabel>
        + From<TypeMismatch>
        + From<KindMismatch>
        + From<<Ty as Kindcheck<Ty>>::CheckError>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let ty_norm = self.ty.clone().normalize(env.clone());
        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ty().normalize(env.clone());
        let term_knd = term_ty.check_kind(env.clone())?;

        let var_ty = ty_norm.clone().into_variant()?;
        let lb_ty = var_ty
            .variants
            .get(&self.label)
            .cloned()
            .ok_or(UndefinedLabel::new(&self.label))?;
        let lb_knd = lb_ty.check_kind(env.clone())?;

        lb_knd.check_equal(&term_knd)?;
        lb_ty.check_equal(&term_ty)?;

        let conc = Conclusion::new(env, self.clone(), ty_norm);
        let deriv = Derivation::variant(conc, term_res);
        Ok(deriv)
    }
}
