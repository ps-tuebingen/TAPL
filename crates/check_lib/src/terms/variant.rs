use crate::{errors::UndefinedLabel, Kindcheck, Normalize, Typecheck};
use syntax::{
    terms::{Term, Variant},
    types::{TypeGroup, Variant as VariantTy},
};

impl<T, Ty> Typecheck for Variant<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: TypeGroup
        + Normalize<Ty, Env = <T as Typecheck>::Env>
        + Kindcheck<Ty, Env = <T as Typecheck>::Env, CheckError = <T as Typecheck>::CheckError>,
    VariantTy<Ty>: Into<Ty>,
    <T as Typecheck>::CheckError: From<UndefinedLabel> + From<syntax::errors::Error>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        let ty_norm = self.ty.clone().normalize(&mut env.clone());
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let term_knd = term_ty.check_kind(&mut env.clone())?;

        let var_ty = ty_norm.clone().into_variant()?;
        let lb_ty = var_ty
            .variants
            .get(&self.label)
            .cloned()
            .ok_or(UndefinedLabel::new(&self.label))?;
        let lb_knd = lb_ty.check_kind(env)?;

        lb_knd.check_equal(&term_knd)?;
        lb_ty.check_equal(&term_ty)?;
        Ok(ty_norm)
    }
}
