use crate::{Subtypecheck, errors::CheckError};
use common::errors::UndefinedLabel;
use syntax::{
    env::Environment,
    types::{TypeGroup, Variant},
};

impl<Ty> Subtypecheck<Ty> for Variant<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty>,
{
    fn check_subtype(&self, sup: &Ty, env: Environment<Ty>) -> Result<(), CheckError> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        let sup_var = sup.clone().into_variant()?;
        for (lb, ty) in sup_var.variants.iter() {
            let self_ty = self.variants.get(lb).ok_or(UndefinedLabel::new(lb))?;
            self_ty.check_subtype(ty, env.clone())?;
        }
        Ok(())
    }
}
