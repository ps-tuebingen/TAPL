use super::Type;
use crate::{errors::TypeKind, subst::SubstType, TypeVar};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Mu<Ty>
where
    Ty: Type,
{
    pub var: TypeVar,
    pub ty: Box<Ty>,
}

impl<Ty> Mu<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(v: &str, ty: Ty1) -> Mu<Ty>
    where
        Ty1: Into<Ty>,
    {
        Mu {
            var: v.to_owned(),
            ty: Box::new(ty.into()),
        }
    }
}

impl<Ty> Type for Mu<Ty>
where
    Ty: Type,
{
    fn knd(&self) -> TypeKind {
        TypeKind::Mu
    }
}

impl<Ty> SubstType<Ty> for Mu<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        if *v == self.var {
            self.into()
        } else {
            Mu {
                var: self.var,
                ty: Box::new(self.ty.subst_type(v, ty)),
            }
            .into()
        }
    }
}

impl<Ty> fmt::Display for Mu<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "mu {}.{}", self.var, self.ty)
    }
}
