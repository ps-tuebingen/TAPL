use super::Type;
use crate::{
    check::{to_subty_err, Kindcheck, Subtypecheck},
    errors::Error,
    eval::Normalize,
    kinds::Kind,
    language::LanguageType,
    subst::SubstType,
    TypeVar,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nat<Ty>
where
    Ty: Type,
{
    phantom: PhantomData<Ty>,
}

impl<Ty> Nat<Ty>
where
    Ty: Type,
{
    pub fn new() -> Nat<Ty> {
        Nat {
            phantom: PhantomData,
        }
    }
}

impl<Ty> Default for Nat<Ty>
where
    Ty: Type,
{
    fn default() -> Nat<Ty> {
        Nat::new()
    }
}

impl<Ty> Type for Nat<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for Nat<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, _: &TypeVar, _: &Ty) -> Self::Target {
        self.into()
    }
}

impl<Ty> Subtypecheck<Ty> for Nat<Ty>
where
    Ty: LanguageType,
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
    Ty: LanguageType,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;

    fn check_kind(&self, _: &mut Self::Env) -> Result<Kind, Error> {
        Ok(Kind::Star)
    }
}
impl<Ty> Normalize<Ty> for Nat<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, _: &mut Self::Env) -> Ty {
        self.into()
    }
}

impl<Ty> fmt::Display for Nat<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Nat")
    }
}
