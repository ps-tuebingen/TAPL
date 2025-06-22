use super::Type;
use crate::{TypeVar, subst::SubstType};
use common::errors::TypeKind;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sink<Ty>
where
    Ty: Type,
{
    pub ty: Box<Ty>,
}

impl<Ty> Sink<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(ty: Ty1) -> Sink<Ty>
    where
        Ty1: Into<Ty>,
    {
        Sink {
            ty: Box::new(ty.into()),
        }
    }
}

impl<Ty> Type for Sink<Ty>
where
    Ty: Type,
{
    fn knd(&self) -> TypeKind {
        TypeKind::Sink
    }
}

impl<Ty> SubstType<Ty> for Sink<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Sink {
            ty: Box::new(self.ty.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Ty> fmt::Display for Sink<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sink[{}]", self.ty)
    }
}
