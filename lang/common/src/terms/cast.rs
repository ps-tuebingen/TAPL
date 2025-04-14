use super::Term;
use crate::{subst::SubstType, types::Type, TypeVar};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Cast<T, Ty>
where
    T: Term,
    Ty: Type,
{
    term: Box<T>,
    ty: Ty,
}

impl<T, Ty> Term for Cast<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> SubstType<Ty> for Cast<T, Ty>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Cast {
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T, Ty> fmt::Display for Cast<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})({})", self.term, self.ty)
    }
}
