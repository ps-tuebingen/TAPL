use super::Type;
use crate::{errors::TypeKind, subst::SubstType, TypeVar};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fun<Ty>
where
    Ty: Type,
{
    pub from: Box<Ty>,
    pub to: Box<Ty>,
}

impl<Ty> Fun<Ty>
where
    Ty: Type,
{
    pub fn new<T: Into<Ty>, U: Into<Ty>>(from: T, to: U) -> Fun<Ty> {
        Fun {
            from: Box::new(from.into()),
            to: Box::new(to.into()),
        }
    }
}

impl<Ty> Type for Fun<Ty>
where
    Ty: Type,
{
    fn knd(&self) -> TypeKind {
        TypeKind::Function
    }
}

impl<Ty> SubstType<Ty> for Fun<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Fun {
            from: Box::new(self.from.subst_type(v, ty)),
            to: Box::new(self.to.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Ty> fmt::Display for Fun<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} -> {})", self.from, self.to)
    }
}
