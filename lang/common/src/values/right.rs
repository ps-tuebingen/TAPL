use super::Value;
use crate::types::Type;

pub struct Right<V, Ty>
where
    V: Value,
    Ty: Type,
{
    right_val: V,
    ty: Ty,
}

impl<V, Ty> Value for Right<V, Ty>
where
    V: Value,
    Ty: Type,
{
}
