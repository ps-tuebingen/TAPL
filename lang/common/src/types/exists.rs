use super::Type;
use crate::TypeVar;

pub struct Exists<Ty>
where
    Ty: Type,
{
    var: TypeVar,
    ty: Box<Ty>,
}

impl<Ty> Type for Exists<Ty> where Ty: Type {}
