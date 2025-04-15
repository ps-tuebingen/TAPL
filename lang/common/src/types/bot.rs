use super::{Fun, Type};
use crate::{errors::ErrorKind, subst::SubstType, TypeVar};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bot;

impl Type for Bot {
    fn into_fun<Ty>(self) -> Result<Fun<Ty>, ErrorKind>
    where
        Ty: Type,
    {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Function Type".to_owned(),
        })
    }
}

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
