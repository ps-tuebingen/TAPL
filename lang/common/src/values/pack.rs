use super::Value;
use crate::types::Type;

pub struct Pack<V, Ty>
where
    V: Value,
    Ty: Type,
{
    inner_ty: Ty,
    val: Box<V>,
    outer_ty: Ty,
}

impl<V, Ty> Value for Pack<V, Ty>
where
    V: Value,
    Ty: Type,
{
}
