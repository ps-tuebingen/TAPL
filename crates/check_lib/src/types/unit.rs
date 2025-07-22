use crate::{Kindcheck, Normalize, Subtypecheck};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{TypeGroup, Unit},
};
impl<Ty> Kindcheck<Ty> for Unit<Ty>
where
    Ty: TypeGroup + Kindcheck<Ty>,
{
    fn check_kind(&self, _: Environment<Ty>) -> Result<Kind, CheckError> {
        Ok(Kind::Star)
    }
}

impl<Ty> Subtypecheck<Ty> for Unit<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty>,
{
    fn check_subtype(&self, sup: &Ty, _: Environment<Ty>) -> Result<(), CheckError> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        sup.clone().into_unit()?;
        Ok(())
    }
}

impl<Ty> Normalize<Ty> for Unit<Ty>
where
    Ty: TypeGroup + Normalize<Ty>,
    Self: Into<Ty>,
{
    fn normalize(self, _: Environment<Ty>) -> Ty {
        self.into()
    }
}
