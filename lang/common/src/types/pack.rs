use super::Type;
use crate::TypeVar;

pub struct Pack<Ty>
where
    Ty: Type,
{
    var: TypeVar,
    ty: Box<Ty>,
}

impl<Ty> Type for Pack<Ty> where Ty: Type {}
