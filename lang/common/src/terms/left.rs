use super::Term;
use crate::types::Type;

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
