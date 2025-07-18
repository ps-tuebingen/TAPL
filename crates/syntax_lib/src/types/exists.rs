use super::{ExistsBounded, Top, Type};
use crate::{TypeVar, kinds::Kind, subst::SubstType};
use errors::TypeKind;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exists<Ty>
where
    Ty: Type,
{
    pub var: TypeVar,
    pub kind: Kind,
    pub ty: Box<Ty>,
}

impl<Ty> Exists<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(v: &str, knd: Kind, ty: Ty1) -> Exists<Ty>
    where
        Ty1: Into<Ty>,
    {
        Exists {
            var: v.to_owned(),
            kind: knd,
            ty: Box::new(ty.into()),
        }
    }

    pub fn to_exists_bounded(self) -> ExistsBounded<Ty>
    where
        Top<Ty>: Into<Ty>,
    {
        ExistsBounded::new_unbounded(&self.var, self.kind, *self.ty)
    }
}

impl<Ty> Type for Exists<Ty>
where
    Ty: Type,
{
    fn knd(&self) -> TypeKind {
        TypeKind::Existential
    }
}

impl<Ty> SubstType<Ty> for Exists<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;

    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        if *v == self.var {
            self.into()
        } else {
            Exists {
                var: self.var,
                kind: self.kind,
                ty: Box::new((*self.ty).subst_type(v, ty)),
            }
            .into()
        }
    }
}

impl<Ty> fmt::Display for Exists<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{exists {}::{},{}}}", self.var, self.kind, self.ty)
    }
}
