use crate::{errors::CheckError, Kindcheck, Subtypecheck};
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{Bot, Type, TypeGroup},
};

impl<Ty> Subtypecheck<Ty> for Bot
where
    Ty: TypeGroup + Subtypecheck<Ty>,
{
    fn check_subtype(&self, _: &Ty, _: Environment<Ty>) -> Result<(), CheckError<Ty>> {
        Ok(())
    }
}

impl<Ty> Kindcheck<Ty> for Bot
where
    Ty: Type + Kindcheck<Ty>,
{
    fn check_kind(&self, _: Environment<Ty>) -> Result<Kind, CheckError<Ty>> {
        Ok(self.kind.clone())
    }
}
