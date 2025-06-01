use crate::{Kindcheck, Subtypecheck};
use syntax::{
    kinds::Kind,
    types::{Bot, Type, TypeGroup},
};

impl<Ty> Subtypecheck<Ty> for Bot
where
    Ty: TypeGroup + Subtypecheck<Ty>,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    type CheckError = <Ty as Subtypecheck<Ty>>::CheckError;

    fn check_subtype(&self, _: &Ty, _: &mut Self::Env) -> Result<(), Self::CheckError> {
        Ok(())
    }
}

impl<Ty> Kindcheck<Ty> for Bot
where
    Ty: Type + Kindcheck<Ty>,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;
    type CheckError = <Ty as Kindcheck<Ty>>::CheckError;

    fn check_kind(&self, _: &mut Self::Env) -> Result<Kind, Self::CheckError> {
        Ok(self.kind.clone())
    }
}
