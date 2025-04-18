use super::Type;
use crate::{
    check::{to_subty_err, Subtypecheck},
    errors::Error,
    language::LanguageType,
    subst::SubstType,
    TypeVar,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bool;

impl Type for Bool {}

impl<Ty> SubstType<Ty> for Bool
where
    Ty: Type,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, _: &TypeVar, _: &Ty) -> Self::Target {
        self.into()
    }
}

impl<Ty> Subtypecheck<Ty> for Bool
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

impl fmt::Display for Bool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Bool")
    }
}
