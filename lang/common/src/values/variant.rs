use super::Value;
use crate::{types::Type, Label};

pub struct Variant<V, Ty>
where
    V: Value,
    Ty: Type,
{
    label: Label,
    val: Box<V>,
    ty: Ty,
}

impl<V, Ty> Value for Variant<V, Ty> {}
