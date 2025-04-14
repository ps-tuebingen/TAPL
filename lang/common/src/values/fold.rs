use super::Value;
use crate::types::Type;

pub struct Fold<V, Ty>
where
    V: Value,
    Ty: Type,
{
    ty: Ty,
    val: Box<V>,
}

impl<V, Ty> Value for Fold<V, Ty>
where
    V: Value,
    Ty: Type,
{
}
