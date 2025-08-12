use super::Type;
use crate::{TypeVar, kinds::Kind, subst::SubstType};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bot {
    pub kind: Kind,
}

impl Bot {
    pub fn new() -> Bot {
        Bot { kind: Kind::Star }
    }
}

impl Default for Bot {
    fn default() -> Bot {
        Bot::new()
    }
}

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
