use super::Type;
use crate::{
    check::{to_subty_err, Subtypecheck},
    errors::{Error, ErrorKind},
    language::LanguageType,
    subst::SubstType,
    TypeVar,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Top;

impl Type for Top {}

impl<Ty> SubstType<Ty> for Top
where
    Ty: Type,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, _: &TypeVar, _: &Ty) -> Self::Target {
        self.into()
    }
}

impl<Ty> Subtypecheck<Ty> for Top
where
    Ty: LanguageType,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    fn check_supertype(&self, _: &Ty, _: &mut Self::Env) -> Result<(), Error> {
        Ok(())
    }
    fn check_subtype(&self, sup: &Ty, _: &mut Self::Env) -> Result<(), Error> {
        Err(to_subty_err(ErrorKind::Subtype {
            sub: self.to_string(),
            sup: sup.to_string(),
        }))
    }
}

impl fmt::Display for Top {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Top")
    }
}
