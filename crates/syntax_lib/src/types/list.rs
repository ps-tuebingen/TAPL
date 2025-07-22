use super::Type;
use crate::{TypeVar, subst::SubstType};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct List<Ty>
where
    Ty: Type,
{
    pub ty: Box<Ty>,
}

impl<Ty> List<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(ty: Ty1) -> List<Ty>
    where
        Ty1: Into<Ty>,
    {
        List {
            ty: Box::new(ty.into()),
        }
    }
}

impl<Ty> Type for List<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for List<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        List {
            ty: Box::new(self.ty.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Ty> fmt::Display for List<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "List[{}]", self.ty)
    }
}
