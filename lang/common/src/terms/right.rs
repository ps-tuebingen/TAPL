use super::Term;
use crate::types::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Right<T, Ty>
where
    T: Term,
    Ty: Type,
{
    right_term: Box<T>,
    ty: Ty,
}

impl<T, Ty> Term for Right<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> fmt::Display for Right<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inl({}) as {}", self.right_term, self.ty)
    }
}
