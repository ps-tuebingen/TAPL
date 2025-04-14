use super::Term;
use crate::types::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct IsNil<T, Ty>
where
    T: Term,
    Ty: Type,
{
    term: Box<T>,
    ty: Ty,
}

impl<T, Ty> Term for IsNil<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> fmt::Display for IsNil<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "isnil[{}]({})", self.ty, self.term)
    }
}
