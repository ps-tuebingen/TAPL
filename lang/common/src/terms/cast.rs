use super::Term;
use crate::types::Type;

pub struct Cast<T, Ty>
where
    T: Term,
    Ty: Type,
{
    term: T,
    ty: Ty,
}
