use super::Term;
use crate::types::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Unfold<T, Ty>
where
    T: Term,
    Ty: Type,
{
    ty: Ty,
    term: Box<T>,
}

impl<T, Ty> Term for Unfold<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> fmt::Display for Unfold<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unfold[{}]({})", self.ty, self.term)
    }
}
