use super::{Fun, Type};
use crate::{errors::ErrorKind, subst::SubstType, TypeVar};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unit;

impl Type for Unit {
    fn into_fun<Ty1>(self) -> Result<Fun<Ty1>, ErrorKind>
    where
        Ty1: Type,
    {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Function Type".to_owned(),
        })
    }
}

impl<Ty> SubstType<Ty> for Unit
where
    Ty: Type,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, _: &TypeVar, _: &Ty) -> Self::Target {
        self.into()
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Unit")
    }
}
