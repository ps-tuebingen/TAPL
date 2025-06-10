use crate::{Kindcheck, Subtypecheck};
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{Bot, Type, TypeGroup},
};

impl<Ty> Subtypecheck<Ty> for Bot
where
    Ty: TypeGroup + Subtypecheck<Ty>,
{
    type CheckError = <Ty as Subtypecheck<Ty>>::CheckError;

    fn check_subtype(&self, _: &Ty, _: Environment<Ty>) -> Result<(), Self::CheckError> {
        Ok(())
    }
}

impl<Ty> Kindcheck<Ty> for Bot
where
    Ty: Type + Kindcheck<Ty>,
{
    type CheckError = <Ty as Kindcheck<Ty>>::CheckError;

    fn check_kind(&self, _: Environment<Ty>) -> Result<Kind, Self::CheckError> {
        Ok(self.kind.clone())
    }
}
