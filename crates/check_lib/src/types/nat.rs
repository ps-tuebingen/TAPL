use crate::{Kindcheck, Normalize, Subtypecheck};
use common::errors::TypeMismatch;
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{Nat, Type, TypeGroup},
};

impl<Ty> Subtypecheck<Ty> for Nat<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty>,
    <Ty as Subtypecheck<Ty>>::CheckError: From<TypeMismatch>,
{
    type CheckError = <Ty as Subtypecheck<Ty>>::CheckError;

    fn check_subtype(&self, sup: &Ty, _: Environment<Ty>) -> Result<(), Self::CheckError> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        sup.clone().into_nat()?;
        Ok(())
    }
}

impl<Ty> Kindcheck<Ty> for Nat<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    type CheckError = <Ty as Kindcheck<Ty>>::CheckError;

    fn check_kind(&self, _: Environment<Ty>) -> Result<Kind, Self::CheckError> {
        Ok(Kind::Star)
    }
}

impl<Ty> Normalize<Ty> for Nat<Ty>
where
    Ty: Type + Normalize<Ty>,
    Self: Into<Ty>,
{
    fn normalize(self, _: Environment<Ty>) -> Ty {
        self.into()
    }
}
