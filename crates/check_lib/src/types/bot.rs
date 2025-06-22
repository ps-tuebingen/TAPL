use crate::{Kindcheck, Subtypecheck, errors::CheckError};
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{Bot, Type, TypeGroup},
};

impl<Ty> Subtypecheck<Ty> for Bot
where
    Ty: TypeGroup + Subtypecheck<Ty>,
{
    fn check_subtype(&self, _: &Ty, _: Environment<Ty>) -> Result<(), CheckError> {
        Ok(())
    }
}

impl<Ty> Kindcheck<Ty> for Bot
where
    Ty: Type + Kindcheck<Ty>,
{
    fn check_kind(&self, _: Environment<Ty>) -> Result<Kind, CheckError> {
        Ok(self.kind.clone())
    }
}
