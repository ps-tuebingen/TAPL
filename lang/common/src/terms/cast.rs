use super::Term;
use crate::types::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Cast<T, Ty>
where
    T: Term,
    Ty: Type,
{
    term: T,
    ty: Ty,
}

impl<T, Ty> fmt::Display for Cast<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})({})", self.term, self.ty)
    }
}
