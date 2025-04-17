use super::Type;
use crate::{subst::SubstType, TypeVar};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Optional<Ty>
where
    Ty: Type,
{
    pub ty: Box<Ty>,
}

impl<Ty> Optional<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(ty: Ty1) -> Optional<Ty>
    where
        Ty1: Into<Ty>,
    {
        Optional {
            ty: Box::new(ty.into()),
        }
    }
}

impl<Ty> Type for Optional<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for Optional<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Optional {
            ty: Box::new(self.ty.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Ty> fmt::Display for Optional<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Optional[{}]", self.ty)
    }
}
