use super::Term;
use crate::types::Type;

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
