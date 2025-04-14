use super::Type;
use std::fmt;
use crate::TypeVar;

#[derive(Clone, Debug)]
pub struct Pack<Ty>
where
    Ty: Type,
{
    var: TypeVar,
    ty: Box<Ty>,
}

impl<Ty> Type for Pack<Ty> where Ty: Type {}

impl<Ty> fmt::Display for Pack<Ty> where Ty:Type{
    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result{
        write!(f,"")
    }
}
