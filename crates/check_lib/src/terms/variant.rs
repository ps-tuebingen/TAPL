use crate::{to_check_err, Kindcheck, Normalize, Typecheck};
use common::errors::{Error, ErrorKind};
use syntax::{
    terms::{Term, Variant},
    types::{TypeGroup, Variant as VariantTy},
};

impl<T, Ty> Typecheck for Variant<T, Ty>
where
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty>,
    T: Term + Typecheck<Type = Ty>,
    VariantTy<Ty>: Into<Ty>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let ty_norm = self.ty.clone().normalize(&mut env.clone());
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let term_knd = term_ty.check_kind(&mut env.clone())?;

        let var_ty = ty_norm.clone().into_variant().map_err(to_check_err)?;
        let lb_ty = var_ty
            .variants
            .get(&self.label)
            .ok_or(to_check_err(ErrorKind::UndefinedLabel(self.label.clone())))
            .cloned()?;
        let lb_knd = lb_ty.check_kind(env)?;

        lb_knd.check_equal(&term_knd).map_err(to_check_err)?;
        lb_ty.check_equal(&term_ty).map_err(to_check_err)?;
        Ok(ty_norm)
    }
}
