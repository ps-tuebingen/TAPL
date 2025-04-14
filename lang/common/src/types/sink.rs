use super::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Sink<Ty>
where
    Ty: Type,
{
    ty: Box<Ty>,
}

impl<Ty> Type for Sink<Ty> where Ty: Type {}

impl<Ty> fmt::Display for Sink<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sink({})", self.ty)
    }
}
