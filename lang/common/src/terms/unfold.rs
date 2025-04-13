use super::Term;
use crate::types::Type;

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
