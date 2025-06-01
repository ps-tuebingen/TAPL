use crate::{errors::NotASubtype, Kindcheck, Normalize, Subtypecheck};
use syntax::{
    kinds::Kind,
    types::{Top, Type, TypeGroup},
};

impl<Ty> Subtypecheck<Ty> for Top<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty> + From<Self>,
    <Ty as Subtypecheck<Ty>>::CheckError: From<NotASubtype<Ty, Ty>>,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    type CheckError = <Ty as Subtypecheck<Ty>>::CheckError;

    fn check_subtype(&self, sup: &Ty, _: &mut Self::Env) -> Result<(), Self::CheckError> {
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
    type Env = <Ty as Kindcheck<Ty>>::Env;
    type CheckError = <Ty as Kindcheck<Ty>>::CheckError;

    fn check_kind(&self, _: &mut Self::Env) -> Result<Kind, Self::CheckError> {
        Ok(self.kind.clone())
    }
}

impl<Ty> Normalize<Ty> for Top<Ty>
where
    Ty: Type + Normalize<Ty>,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, _: &mut Self::Env) -> Ty {
        self.into()
    }
}
