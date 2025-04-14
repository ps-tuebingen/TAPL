use super::Value;
use crate::types::Type;

pub struct Nil<Ty>
where
    Ty: Type,
{
    ty: Ty,
}

impl<Ty> Value for Nil<Ty> {}
