use super::{Fun, Type};
use crate::{errors::ErrorKind, subst::SubstType, TypeVar};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sum<Ty>
where
    Ty: Type,
{
    left: Box<Ty>,
    right: Box<Ty>,
}

impl<Ty> Type for Sum<Ty>
where
    Ty: Type,
{
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

impl<Ty> SubstType<Ty> for Sum<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Sum {
            left: Box::new(self.left.subst_type(v, ty)),
            right: Box::new(self.right.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Ty> fmt::Display for Sum<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) + ({})", self.left, self.right)
    }
}
