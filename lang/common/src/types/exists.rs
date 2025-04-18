use super::Type;
use crate::{subst::SubstType, TypeVar};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exists<Ty>
where
    Ty: Type,
{
    pub var: TypeVar,
    pub ty: Box<Ty>,
}

impl<Ty> Exists<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(v: &str, ty: Ty1) -> Exists<Ty>
    where
        Ty1: Into<Ty>,
    {
        Exists {
            var: v.to_owned(),
            ty: Box::new(ty.into()),
        }
    }
}

impl<Ty> Type for Exists<Ty> where Ty: Type {}

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
        write!(f, "{{exitsts {},{}}}", self.var, self.ty)
    }
}
