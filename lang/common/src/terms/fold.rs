use super::Term;
use crate::types::Type;

pub struct Fold<T, Ty>
where
    T: Term,
    Ty: Type,
{
    inner_type: Ty,
    term: T,
    outer_type: Ty,
}
