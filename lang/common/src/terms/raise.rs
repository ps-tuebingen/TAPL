use super::Term;
use crate::types::Type;

pub struct Raise<T, Ty>
where
    T: Term,
    Ty: Type,
{
    exception: Box<T>,
    exception_ty: Box<Ty>,
    cont_ty: Box<Ty>,
}

impl<T, Ty> Term for Raise<T, Ty>
where
    T: Term,
    Ty: Type,
{
}
