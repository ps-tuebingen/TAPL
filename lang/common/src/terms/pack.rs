use super::Term;
use crate::types::Type;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Pack<T, Ty>
where
    T: Term,
    Ty: Type,
{
    inner_ty: Ty,
    term: Box<T>,
    outer_ty: Ty,
}

impl<T, Ty> Term for Pack<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> fmt::Display for Pack<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{*{},{}}} as {}",
            self.inner_ty, self.term, self.outer_ty
        )
    }
}
