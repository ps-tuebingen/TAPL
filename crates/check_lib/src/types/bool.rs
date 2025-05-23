use crate::{to_subty_err, Kindcheck, Subtypecheck};
use common::errors::Error;
use syntax::{
    kinds::Kind,
    types::{Bool, Type, TypeGroup},
};

impl<Ty> Subtypecheck<Ty> for Bool<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty>,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;

    fn check_subtype(&self, sup: &Ty, _: &mut Self::Env) -> Result<(), Error> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        sup.clone().into_bool().map(|_| ()).map_err(to_subty_err)
    }
}

impl<Ty> Kindcheck<Ty> for Bool<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;

    fn check_kind(&self, _: &mut Self::Env) -> Result<Kind, Error> {
        Ok(Kind::Star)
    }
}
