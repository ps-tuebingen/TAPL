use super::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Source<Ty>
where
    Ty: Type,
{
    ty: Box<Ty>,
}

impl<Ty> Type for Source<Ty> where Ty: Type {}

impl<Ty> fmt::Display for Source<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Source({})", self.ty)
    }
}
