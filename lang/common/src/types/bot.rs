use super::Type;
use crate::{subst::SubstType, TypeVar};
use std::fmt;

#[derive(Debug, Clone)]
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

impl fmt::Display for Bot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Bot")
    }
}
