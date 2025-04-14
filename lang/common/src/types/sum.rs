use super::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Sum<Ty>
where
    Ty: Type,
{
    left: Box<Ty>,
    right: Box<Ty>,
}

impl<Ty> Type for Sum<Ty> where Ty: Type {}

impl<Ty> fmt::Display for Sum<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) + ({})", self.left, self.right)
    }
}
