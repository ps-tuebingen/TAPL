use super::Type;
use crate::TypeVar;

pub struct Forall<Ty>
where
    Ty: Type,
{
    var: TypeVar,
    ty: Box<Ty>,
}

impl<Ty> Type for Forall<Ty> where Ty: Type {}
