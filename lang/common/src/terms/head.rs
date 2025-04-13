use super::Term;
use crate::types::Type;

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
