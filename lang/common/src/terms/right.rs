use super::Term;
use crate::types::Type;

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
