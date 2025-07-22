use super::Type;
use crate::{TypeVar, subst::SubstType};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Product<Ty>
where
    Ty: Type,
{
    pub fst: Box<Ty>,
    pub snd: Box<Ty>,
}

impl<Ty> Product<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1, Ty2>(fst: Ty1, snd: Ty2) -> Product<Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        Product {
            fst: Box::new(fst.into()),
            snd: Box::new(snd.into()),
        }
    }
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
        write!(f, "({} x {})", self.fst, self.snd)
    }
}
