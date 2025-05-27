use crate::{Kindcheck, Subtypecheck};
use common::errors::Error;
use syntax::types::{Source, TypeGroup};

impl<Ty> Subtypecheck<Ty> for Source<Ty>
where
    Ty: TypeGroup,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;

    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        let sup_src = sup.clone().into_source().map_err(to_subty_err)?;
        self.ty.check_subtype(&(*sup_src.ty), env)
    }
}
