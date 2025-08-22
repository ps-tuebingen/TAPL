use super::Type;
use crate::{TypeVar, language::Language, subst::SubstType};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Source<Lang>
where
    Lang: Language,
{
    pub ty: Box<Lang::Type>,
}

impl<Lang> Source<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1>(ty: Ty1) -> Source<Lang>
    where
        Ty1: Into<Lang::Type>,
    {
        Source {
            ty: Box::new(ty.into()),
        }
    }
}

impl<Lang> Type for Source<Lang> where Lang: Language {}

impl<Lang> SubstType for Source<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Source {
            ty: Box::new(self.ty.subst_type(v, ty)),
        }
    }
}

impl<Lang> fmt::Display for Source<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Source[{}]", self.ty)
    }
}
