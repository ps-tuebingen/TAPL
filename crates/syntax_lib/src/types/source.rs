use super::Type;
use crate::{subst::SubstType, TypeVar};
use common::errors::TypeKind;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Source<Ty>
where
    Ty: Type,
{
    pub ty: Box<Ty>,
}

impl<Ty> Source<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(ty: Ty1) -> Source<Ty>
    where
        Ty1: Into<Ty>,
    {
        Source {
            ty: Box::new(ty.into()),
        }
    }
}

impl<Ty> Type for Source<Ty>
where
    Ty: Type,
{
    fn knd(&self) -> TypeKind {
        TypeKind::Source
    }
}

impl<Ty> SubstType<Ty> for Source<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Source {
            ty: Box::new(self.ty.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Ty> fmt::Display for Source<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Source({})", self.ty)
    }
}
