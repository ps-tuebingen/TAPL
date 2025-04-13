use super::Term;
use crate::types::Type;

pub struct Tail<T, Ty>
where
    T: Term,
    Ty: Type,
{
    term: Box<T>,
    ty: Ty,
}

impl<T, Ty> Term for Tail<T, Ty>
where
    T: Term,
    Ty: Type,
{
}
