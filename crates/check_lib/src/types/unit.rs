use crate::{to_subty_err, Kindcheck, Normalize, Subtypecheck};
use common::errors::Error;
use syntax::{
    kinds::Kind,
    types::{TypeGroup, Unit},
};

impl<Ty> Kindcheck<Ty> for Unit<Ty>
where
    Ty: TypeGroup + Kindcheck<Ty>,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;

    fn check_kind(&self, _: &mut Self::Env) -> Result<Kind, Error> {
        Ok(Kind::Star)
    }
}

impl<Ty> Subtypecheck<Ty> for Unit<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty>,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;

    fn check_subtype(&self, sup: &Ty, _: &mut Self::Env) -> Result<(), Error> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        sup.clone().into_unit().map(|_| ()).map_err(to_subty_err)
    }
}

impl<Ty> Normalize<Ty> for Unit<Ty>
where
    Ty: TypeGroup + Normalize<Ty>,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, _: &mut Self::Env) -> Ty {
        self.into()
    }
}
