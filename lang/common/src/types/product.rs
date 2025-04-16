use super::Type;
use crate::{subst::SubstType, TypeVar};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Product<Ty>
where
    Ty: Type,
{
    fst: Box<Ty>,
    snd: Box<Ty>,
}

impl<Ty> Type for Product<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for Product<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;

    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Product {
            fst: Box::new(self.fst.subst_type(v, ty)),
            snd: Box::new(self.snd.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Ty> fmt::Display for Product<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) x ({})", self.fst, self.snd)
    }
}
