use super::Type;
use crate::TypeVar;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Forall<Ty>
where
    Ty: Type,
{
    var: TypeVar,
    ty: Box<Ty>,
}

impl<Ty> Type for Forall<Ty> where Ty: Type {}

impl<Ty> fmt::Display for Forall<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "forall {}.{}", self.var, self.ty)
    }
}
