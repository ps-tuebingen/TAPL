use super::Type;
use crate::{subst::SubstType, TypeVar};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sum<Ty>
where
    Ty: Type,
{
    pub left: Box<Ty>,
    pub right: Box<Ty>,
}

impl<Ty> Sum<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1, Ty2>(l: Ty1, r: Ty2) -> Sum<Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        Sum {
            left: Box::new(l.into()),
            right: Box::new(r.into()),
        }
    }
}

impl<Ty> Type for Sum<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for Sum<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Sum {
            left: Box::new(self.left.subst_type(v, ty)),
            right: Box::new(self.right.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Ty> fmt::Display for Sum<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}+{})", self.left, self.right)
    }
}
