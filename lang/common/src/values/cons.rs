use super::Value;
use crate::types::Type;

pub struct Cons<V, Ty>
where
    V: Value,
    Ty: Type,
{
    fst_val: V,
    rst_val: V,
    ty: Ty,
}

impl<V, Ty> Value for Cons<V, Ty>
where
    V: Value,
    Ty: Type,
{
}
