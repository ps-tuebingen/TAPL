use crate::{Kindcheck, Subtypecheck};
use common::errors::Error;
use syntax::types::{List, TypeGroup};

impl<Ty> Subtypecheck<Ty> for List<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        let sup_list = sup.clone().into_list().map_err(to_subty_err)?;
        self.ty.check_subtype(&(*sup_list.ty), env)
    }
}
