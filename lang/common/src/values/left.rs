use super::Value;
use crate::types::Type;

pub struct Left<V, Ty>
where
    V: Value,
    Ty: Type,
{
    left_val: V,
    ty: Ty,
}

impl<V, Ty> Value for Left<V, Ty>
where
    V: Value,
    Ty: Type,
{
}
