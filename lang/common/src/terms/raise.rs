use super::Term;
use crate::types::Type;
use std::fmt;

#[derive(Clone, Debug)]
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

impl<T, Ty> fmt::Display for Raise<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "raise[{}]({} : {})",
            self.cont_ty, self.exception, self.exception_ty
        )
    }
}
