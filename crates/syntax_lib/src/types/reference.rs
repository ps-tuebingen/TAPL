use super::Type;
use crate::{subst::SubstType, TypeVar};
use common::errors::TypeKind;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reference<Ty>
where
    Ty: Type,
{
    pub ty: Box<Ty>,
}

impl<Ty> Reference<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(ty: Ty1) -> Reference<Ty>
    where
        Ty1: Into<Ty>,
    {
        Reference {
            ty: Box::new(ty.into()),
        }
    }
}

impl<Ty> Type for Reference<Ty>
where
    Ty: Type,
{
    fn knd(&self) -> TypeKind {
        TypeKind::Reference
    }
}

impl<Ty> SubstType<Ty> for Reference<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Reference {
            ty: Box::new(self.ty.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Ty> fmt::Display for Reference<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ref({})", self.ty)
    }
}
