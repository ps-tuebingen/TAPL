use crate::{errors::NotASubtype, Kindcheck, Normalize, Subtypecheck};
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{Top, Type, TypeGroup},
};

impl<Ty> Subtypecheck<Ty> for Top<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty> + From<Self>,
    <Ty as Subtypecheck<Ty>>::CheckError: From<NotASubtype<Ty, Ty>>,
{
    type CheckError = <Ty as Subtypecheck<Ty>>::CheckError;

    fn check_subtype(&self, sup: &Ty, _: Environment<Ty>) -> Result<(), Self::CheckError> {
        if sup.clone().into_top().is_ok() {
            Ok(())
        } else {
            Err(NotASubtype::<Ty, Ty>::new(self.clone().into(), sup.clone()).into())
        }
    }
}

impl<Ty> Kindcheck<Ty> for Top<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    type CheckError = <Ty as Kindcheck<Ty>>::CheckError;

    fn check_kind(&self, _: Environment<Ty>) -> Result<Kind, Self::CheckError> {
        Ok(self.kind.clone())
    }
}

impl<Ty> Normalize<Ty> for Top<Ty>
where
    Ty: Type + Normalize<Ty>,
    Self: Into<Ty>,
{
    fn normalize(self, _: Environment<Ty>) -> Ty {
        self.into()
    }
}
