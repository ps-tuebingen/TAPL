use super::Term;
use crate::types::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Head<T, Ty>
where
    T: Term,
    Ty: Type,
{
    term: Box<T>,
    ty: Ty,
}

impl<T, Ty> Term for Head<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> fmt::Display for Head<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "head[{}]({})", self.ty, self.term)
    }
}
