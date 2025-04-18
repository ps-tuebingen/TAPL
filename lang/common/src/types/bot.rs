use super::Type;
use crate::{
    check::{to_subty_err, Subtypecheck},
    errors::{Error, ErrorKind},
    language::LanguageType,
    subst::SubstType,
    TypeVar,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bot;

impl Type for Bot {}

impl<Ty> SubstType<Ty> for Bot
where
    Ty: Type,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, _: &TypeVar, _: &Ty) -> Self::Target {
        self.into()
    }
}

impl<Ty> Subtypecheck<Ty> for Bot
where
    Ty: LanguageType,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    fn check_subtype(&self, _: &Ty, _: &mut Self::Env) -> Result<(), Error> {
        Ok(())
    }

    fn check_supertype(&self, sub: &Ty, _: &mut Self::Env) -> Result<(), Error> {
        Err(to_subty_err(ErrorKind::Subtype {
            sup: self.to_string(),
            sub: sub.to_string(),
        }))
    }
}

impl fmt::Display for Bot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Bot")
    }
}
