use crate::{Kindcheck, Normalize, Subtypecheck};
use common::errors::TypeMismatch;
use syntax::{
    kinds::Kind,
    types::{TypeGroup, Unit},
};
impl<Ty> Kindcheck<Ty> for Unit<Ty>
where
    Ty: TypeGroup + Kindcheck<Ty>,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;
    type CheckError = <Ty as Kindcheck<Ty>>::CheckError;

    fn check_kind(&self, _: &mut Self::Env) -> Result<Kind, Self::CheckError> {
        Ok(Kind::Star)
    }
}

impl<Ty> Subtypecheck<Ty> for Unit<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty>,
    <Ty as Subtypecheck<Ty>>::CheckError: From<TypeMismatch>,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    type CheckError = <Ty as Subtypecheck<Ty>>::CheckError;

    fn check_subtype(&self, sup: &Ty, _: &mut Self::Env) -> Result<(), Self::CheckError> {
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
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, _: &mut Self::Env) -> Ty {
        self.into()
    }
}
