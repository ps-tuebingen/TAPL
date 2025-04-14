use super::Term;
use crate::{subst::SubstType, types::Type, TypeVar};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Ascribe<T, Ty>
where
    T: Term,
    Ty: Type,
{
    term: Box<T>,
    ty: Ty,
}

impl<T, Ty> Term for Ascribe<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> SubstType<Ty> for Ascribe<T, Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    T: Term + SubstType<Ty, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Ascribe {
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T, Ty> fmt::Display for Ascribe<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} as {}", self.term, self.ty)
    }
}
