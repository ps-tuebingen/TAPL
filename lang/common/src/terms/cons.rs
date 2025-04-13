use super::Term;
use crate::types::Type;

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
