use super::Term;
use crate::types::Type;

pub struct TyApp<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fun: Box<T>,
    arg: Ty,
}

impl<T, Ty> Term for TyApp<T, Ty>
where
    T: Term,
    Ty: Type,
{
}
