use super::Term;
use crate::types::Type;

pub struct Pack<T, Ty>
where
    T: Term,
    Ty: Type,
{
    inner_ty: Ty,
    term: Box<T>,
    outer_ty: Ty,
}

impl<T, Ty> Term for Pack<T, Ty>
where
    T: Term,
    Ty: Type,
{
}
