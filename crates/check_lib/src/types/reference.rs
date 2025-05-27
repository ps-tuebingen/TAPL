use crate::{Kindcheck, Subtypecheck};
use common::errors::Error;
use syntax::types::{Reference, TypeGroup};

impl<Ty> Subtypecheck<Ty> for Reference<Ty>
where
    Ty: TypeGroup,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;

    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        if let Ok(src) = sup.clone().into_source() {
            self.ty.check_subtype(&(*src.ty), env)
        } else if let Ok(sink) = sup.clone().into_sink() {
            sink.ty.check_subtype(&(*sink.ty), env)
        } else {
            let sup_ref = sup.clone().into_ref().map_err(to_subty_err)?;
            sup_ref.ty.check_subtype(&(*self.ty), &mut env.clone())?;
            self.ty.check_subtype(&(*sup_ref.ty), env)
        }
    }
}
