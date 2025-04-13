use super::Type;
use crate::TypeVar;

pub struct Mu<Ty>
where
    Ty: Type,
{
    var: TypeVar,
    ty: Box<Ty>,
}

impl<Ty> Type for Mu<Ty> where Ty: Type {}
