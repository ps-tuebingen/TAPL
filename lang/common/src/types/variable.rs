use super::Type;
use crate::{subst::SubstType, TypeVar};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeVariable {
    v: TypeVar,
}

impl TypeVariable {
    pub fn new(v: &str) -> TypeVariable {
        TypeVariable { v: v.to_owned() }
    }
}

impl Type for TypeVariable {}

impl<Ty> SubstType<Ty> for TypeVariable
where
    Ty: Type,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        if *v == self.v {
            ty.clone()
        } else {
            self.into()
        }
    }
}

impl fmt::Display for TypeVariable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.v)
    }
}
