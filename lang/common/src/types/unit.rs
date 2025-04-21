use super::Type;
use crate::{
    check::{to_subty_err, Kindcheck, Subtypecheck},
    errors::Error,
    kinds::Kind,
    language::LanguageType,
    subst::SubstType,
    TypeVar,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unit<Ty>
where
    Ty: Type,
{
    phantom: PhantomData<Ty>,
}

impl<Ty> Unit<Ty>
where
    Ty: Type,
{
    pub fn new() -> Unit<Ty> {
        Unit {
            phantom: PhantomData,
        }
    }
}

impl<Ty> Type for Unit<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for Unit<Ty>
where
    Ty: Type,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, _: &TypeVar, _: &Ty) -> Self::Target {
        self.into()
    }
}

impl<Ty> Subtypecheck<Ty> for Unit<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;

    fn check_supertype(&self, sub: &Ty, _: &mut Self::Env) -> Result<(), Error> {
        if let Ok(_) = sub.clone().into_bot() {
            return Ok(());
        }

        sub.clone().into_bool().map(|_| ()).map_err(to_subty_err)
    }

    fn check_subtype(&self, sup: &Ty, _: &mut Self::Env) -> Result<(), Error> {
        if let Ok(_) = sup.clone().into_top() {
            return Ok(());
        }

        sup.clone().into_bool().map(|_| ()).map_err(to_subty_err)
    }
}

impl<Ty> Kindcheck<Ty> for Unit<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;

    fn check_kind(&self, _: &mut Self::Env) -> Result<Kind, Error> {
        Ok(Kind::Star)
    }
}

impl<Ty> fmt::Display for Unit<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Unit")
    }
}
