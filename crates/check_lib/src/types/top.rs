use crate::{to_subty_err, Kindcheck, Normalize, Subtypecheck};
use common::errors::{Error, ErrorKind};
use syntax::{
    kinds::Kind,
    types::{Top, Type, TypeGroup},
};

impl<Ty> Subtypecheck<Ty> for Top<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty>,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;

    fn check_subtype(&self, sup: &Ty, _: &mut Self::Env) -> Result<(), Error> {
        if sup.clone().into_top().is_ok() {
            Ok(())
        } else {
            Err(to_subty_err(ErrorKind::Subtype {
                sub: self.to_string(),
                sup: sup.to_string(),
            }))
        }
    }
}

impl<Ty> Kindcheck<Ty> for Top<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;
    fn check_kind(&self, _: &mut Self::Env) -> Result<Kind, Error> {
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
