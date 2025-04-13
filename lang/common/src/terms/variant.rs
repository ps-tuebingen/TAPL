use super::Term;
use crate::types::Type;

pub struct Variant<T, Ty>
where
    T: Term,
    Ty: Type,
{
    term: Box<T>,
    ty: Ty,
}

impl<T, Ty> Term for Variant<T, Ty>
where
    T: Term,
    Ty: Type,
{
}
