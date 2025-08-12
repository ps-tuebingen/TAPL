use super::Type;
use crate::{TypeVar, kinds::Kind, subst::SubstType};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Top<Ty>
where
    Ty: Type,
{
    pub kind: Kind,
    phantom: PhantomData<Ty>,
}

impl<Ty> Top<Ty>
where
    Ty: Type,
{
    pub fn new(knd: Kind) -> Top<Ty> {
        Top {
            kind: knd,
            phantom: PhantomData,
        }
    }

    pub fn new_star() -> Top<Ty> {
        Top {
            kind: Kind::Star,
            phantom: PhantomData,
        }
    }
}

impl<Ty> Type for Top<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for Top<Ty>
where
    Ty: Type,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, _: &TypeVar, _: &Ty) -> Self::Target {
        self.into()
    }
}

impl<Ty> fmt::Display for Top<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Top[{}]", self.kind)
    }
}
