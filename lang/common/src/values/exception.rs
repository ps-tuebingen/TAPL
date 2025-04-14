use super::Value;
use crate::types::Type;

pub struct Exception<Ty>
where
    Ty: Type,
{
    ty: Ty,
}

impl<Ty> Value for Exception<Ty> where Ty: Type {}
