use super::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct List<Ty>
where
    Ty: Type,
{
    ty: Box<Ty>,
}

impl<Ty> Type for List<Ty> where Ty: Type {}

impl<Ty> fmt::Display for List<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "List[{}]", self.ty)
    }
}
