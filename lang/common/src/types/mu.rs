use super::Type;
use crate::TypeVar;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Mu<Ty>
where
    Ty: Type,
{
    var: TypeVar,
    ty: Box<Ty>,
}

impl<Ty> Type for Mu<Ty> where Ty: Type {}

impl<Ty> fmt::Display for Mu<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "mu {}.{}", self.var, self.ty)
    }
}
