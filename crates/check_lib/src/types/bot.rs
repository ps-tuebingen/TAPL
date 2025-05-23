use crate::{Kindcheck, Subtypecheck};
use common::errors::Error;
use syntax::{
    kinds::Kind,
    types::{Bot, Type, TypeGroup},
};

impl<Ty> Subtypecheck<Ty> for Bot
where
    Ty: TypeGroup + Subtypecheck<Ty>,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    fn check_subtype(&self, _: &Ty, _: &mut Self::Env) -> Result<(), Error> {
        Ok(())
    }
}

impl<Ty> Kindcheck<Ty> for Bot
where
    Ty: Type + Kindcheck<Ty>,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;

    fn check_kind(&self, _: &mut Self::Env) -> Result<Kind, Error> {
        Ok(self.kind)
    }
}
