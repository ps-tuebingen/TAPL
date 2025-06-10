use crate::{Kindcheck, Normalize, Subtypecheck};
use common::errors::TypeMismatch;
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{TypeGroup, Unit},
};
impl<Ty> Kindcheck<Ty> for Unit<Ty>
where
    Ty: TypeGroup + Kindcheck<Ty>,
{
    type CheckError = <Ty as Kindcheck<Ty>>::CheckError;

    fn check_kind(&self, _: Environment<Ty>) -> Result<Kind, Self::CheckError> {
        Ok(Kind::Star)
    }
}

impl<Ty> Subtypecheck<Ty> for Unit<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty>,
    <Ty as Subtypecheck<Ty>>::CheckError: From<TypeMismatch>,
{
    type CheckError = <Ty as Subtypecheck<Ty>>::CheckError;

    fn check_subtype(&self, sup: &Ty, _: Environment<Ty>) -> Result<(), Self::CheckError> {
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
