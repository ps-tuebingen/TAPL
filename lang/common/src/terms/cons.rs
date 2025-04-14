use super::Term;
use crate::types::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Cons<T, Ty>
where
    T: Term,
    Ty: Type,
{
    head: Box<T>,
    tail: Box<T>,
    ty: Ty,
}

impl<T, Ty> Term for Cons<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> fmt::Display for Cons<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cons[{}]({},{})", self.ty, self.head, self.tail)
    }
}
