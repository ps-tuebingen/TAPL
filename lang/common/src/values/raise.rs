use super::Value;
use crate::types::Type;

pub struct Raise<V, Ty>
where
    V: Value,
    Ty: Type,
{
    val: Box<V>,
    cont_ty: Ty,
    exception_ty: Ty,
}

impl<V, Ty> Value for Raise<V, Ty>
where
    V: Value,
    Ty: Type,
{
}
