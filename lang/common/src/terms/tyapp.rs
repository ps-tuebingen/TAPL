use super::Term;
use crate::types::Type;
use std::fmt;

#[derive(Clone, Debug)]
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

impl<T, Ty> fmt::Display for TyApp<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})[{}]", self.fun, self.arg)
    }
}
