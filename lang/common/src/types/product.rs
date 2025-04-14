use super::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Product<Ty>
where
    Ty: Type,
{
    fst: Box<Ty>,
    snd: Box<Ty>,
}

impl<Ty> fmt::Display for Product<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) x ({})", self.fst, self.snd)
    }
}
