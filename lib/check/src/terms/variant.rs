use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Conclusion, Derivation, TypingDerivation};
use errors::UndefinedLabel;
use errors::check_error::CheckError;
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
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        let ty_norm = self.ty.clone().normalize(env.clone());
        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ret_ty().normalize(env.clone());
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
        let deriv = TypingDerivation::variant(conc, term_res);
        Ok(deriv.into())
    }
}
