use super::Term;
use crate::types::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Left<T, Ty>
where
    T: Term,
    Ty: Type,
{
    left_term: Box<T>,
    ty: Ty,
}

impl<T, Ty> Term for Left<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> fmt::Display for Left<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inl({}) as {}", self.left_term, self.ty)
    }
}
