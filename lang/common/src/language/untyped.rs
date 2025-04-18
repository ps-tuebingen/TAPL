use crate::{
    check::{Subtypecheck, Typecheck},
    errors::Error,
    language::{LanguageTerm, LanguageType},
    subst::SubstType,
    types::Type,
    TypeVar,
};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Untyped;

impl Type for Untyped {}
impl LanguageType for Untyped {}

impl<T> Typecheck for T
where
    T: LanguageTerm<Type = Untyped>,
{
    type Type = Untyped;
    type Env = ();

    fn check(&self, _: &mut Self::Env) -> Result<Self::Type, Error> {
        Ok(Untyped)
    }
}

impl Subtypecheck for Untyped {
    type Env = ();

    fn check_subtype(&self, _: &Self, _: &mut Self::Env) -> Result<(), Error> {
        Ok(())
    }
    fn check_supertype(&self, _: &Self, _: &mut Self::Env) -> Result<(), Error> {
        Ok(())
    }
}

impl<T> SubstType<Untyped> for T
where
    T: LanguageTerm<Type = Untyped>,
{
    type Target = T;
    fn subst_type(self, _: &TypeVar, _: &Untyped) -> Self::Target {
        self
    }
}

impl fmt::Display for Untyped {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("untyped")
    }
}

impl SubstType<Untyped> for Untyped {
    type Target = Self;
    fn subst_type(self, _: &TypeVar, _: &Self) -> Self::Target {
        self
    }
}
