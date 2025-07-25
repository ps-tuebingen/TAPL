use crate::{
    Kindcheck, Normalize, Subtypecheck,
    errors::{CheckError, NotASubtype},
};
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{Top, Type, TypeGroup},
};

impl<Ty> Subtypecheck<Ty> for Top<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty> + From<Self>,
{
    fn check_subtype(&self, sup: &Ty, _: Environment<Ty>) -> Result<(), CheckError> {
        if sup.clone().into_top().is_ok() {
            Ok(())
        } else {
            Err(NotASubtype::new(self.clone(), sup.clone()).into())
        }
    }
}

impl<Ty> Kindcheck<Ty> for Top<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    fn check_kind(&self, _: Environment<Ty>) -> Result<Kind, CheckError> {
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
