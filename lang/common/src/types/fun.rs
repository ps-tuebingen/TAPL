use super::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Fun<Ty>
where
    Ty: Type,
{
    from: Box<Ty>,
    to: Box<Ty>,
}

impl<Ty> Type for Fun<Ty> where Ty: Type {}

impl<Ty> fmt::Display for Fun<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)
    }
}
