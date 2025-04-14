use super::Value;
use crate::types::Type;

pub struct Nothing<Ty>
where
    Ty: Type,
{
    ty: Ty,
}

impl<Ty> Value for Nothing<Ty> where Ty: Type {}
