use crate::{to_subty_err, Subtypecheck};
use common::errors::{Error, ErrorKind};
use syntax::types::{TypeGroup, Variant};

impl<Ty> Subtypecheck<Ty> for Variant<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty>,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;

    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        let sup_var = sup.clone().into_variant().map_err(to_subty_err)?;
        for (lb, ty) in sup_var.variants.iter() {
            let self_ty = self
                .variants
                .get(lb)
                .ok_or(to_subty_err(ErrorKind::UndefinedLabel(lb.clone())))?;
            self_ty.check_subtype(ty, &mut env.clone())?;
        }
        Ok(())
    }
}
