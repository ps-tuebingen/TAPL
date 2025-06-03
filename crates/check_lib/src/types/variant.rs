use crate::Subtypecheck;
use common::errors::{TypeMismatch, UndefinedLabel};
use syntax::{
    env::Environment,
    types::{TypeGroup, Variant},
};

impl<Ty> Subtypecheck<Ty> for Variant<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty>,
    <Ty as Subtypecheck<Ty>>::CheckError: From<TypeMismatch>,
    <Ty as Subtypecheck<Ty>>::CheckError: From<UndefinedLabel>,
{
    type CheckError = <Ty as Subtypecheck<Ty>>::CheckError;

    fn check_subtype(&self, sup: &Ty, env: &mut Environment<Ty>) -> Result<(), Self::CheckError> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        let sup_var = sup.clone().into_variant()?;
        for (lb, ty) in sup_var.variants.iter() {
            let self_ty = self.variants.get(lb).ok_or(UndefinedLabel::new(&lb))?;
            self_ty.check_subtype(ty, &mut env.clone())?;
        }
        Ok(())
    }
}
