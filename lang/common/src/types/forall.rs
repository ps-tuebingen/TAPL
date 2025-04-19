use super::Type;
use crate::{kinds::Kind, subst::SubstType, TypeVar};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forall<Ty>
where
    Ty: Type,
{
    var: TypeVar,
    kind: Kind,
    ty: Box<Ty>,
}

impl<Ty> Forall<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(v: &str, knd: Kind, ty: Ty1) -> Forall<Ty>
    where
        Ty1: Into<Ty>,
    {
        Forall {
            var: v.to_owned(),
            kind: knd,
            ty: Box::new(ty.into()),
        }
    }
}

impl<Ty> Type for Forall<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for Forall<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        if *v == self.var {
            self.into()
        } else {
            Forall {
                var: self.var,
                kind: self.kind,
                ty: Box::new(self.ty.subst_type(v, ty)),
            }
            .into()
        }
    }
}

impl<Ty> fmt::Display for Forall<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "forall {}.{}", self.var, self.ty)
    }
}
