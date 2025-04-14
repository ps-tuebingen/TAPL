use super::Term;
use crate::{subst::SubstType, types::Type, TypeVar};
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

impl<T, Ty> SubstType<Ty> for Pack<T, Ty>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Pack {
            inner_ty: self.inner_ty.subst_type(v, ty),
            term: Box::new(self.term.subst_type(v, ty)),
            outer_ty: self.outer_ty.subst_type(v, ty),
        }
        .into()
    }
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
