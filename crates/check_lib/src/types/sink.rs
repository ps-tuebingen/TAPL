use crate::{to_subty_err, Subtypecheck};
use common::errors::Error;
use syntax::types::{Sink, TypeGroup};

impl<Ty> Subtypecheck<Ty> for Sink<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty>,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;

    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        let sup_sink = sup.clone().into_sink().map_err(to_subty_err)?;
        sup_sink.ty.check_subtype(&(*self.ty), env)
    }
}
