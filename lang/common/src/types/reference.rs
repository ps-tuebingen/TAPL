use super::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Reference<Ty>
where
    Ty: Type,
{
    ty: Box<Ty>,
}

impl<Ty> Type for Reference<Ty> where Ty: Type {}

impl<Ty> fmt::Display for Reference<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ref({})", self.ty)
    }
}
