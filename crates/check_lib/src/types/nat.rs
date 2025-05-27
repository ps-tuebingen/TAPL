use crate::{Kindcheck, Subtypecheck};
use common::errors::Error;
use syntax::types::{Nat, TypeGroup};

impl<Ty> Subtypecheck<Ty> for Nat<Ty>
where
    Ty: TypeGroup,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;

    fn check_subtype(&self, sup: &Ty, _: &mut Self::Env) -> Result<(), Error> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        sup.clone().into_nat().map(|_| ()).map_err(to_subty_err)
    }
}

impl<Ty> Kindcheck<Ty> for Nat<Ty>
where
    Ty: TypeGroup,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;

    fn check_kind(&self, _: &mut Self::Env) -> Result<Kind, Error> {
        Ok(Kind::Star)
    }
}

impl<Ty> Normalize<Ty> for Nat<Ty>
where
    Ty: TypeGroup,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, _: &mut Self::Env) -> Ty {
        self.into()
    }
}
