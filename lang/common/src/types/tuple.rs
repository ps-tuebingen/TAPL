use super::Type;
use crate::{subst::SubstType, TypeVar};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tuple<Ty>
where
    Ty: Type,
{
    tys: Vec<Ty>,
}

impl<Ty> Type for Tuple<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for Tuple<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Tuple {
            tys: self
                .tys
                .into_iter()
                .map(|ty1| ty1.subst_type(v, ty))
                .collect(),
        }
        .into()
    }
}

impl<Ty> fmt::Display for Tuple<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tys: Vec<String> = self.tys.iter().map(|ty| ty.to_string()).collect();
        tys.sort();
        write!(f, "({})", tys.join(", "))
    }
}
