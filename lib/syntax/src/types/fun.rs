use super::Type;
use crate::{TypeVar, language::Language, subst::SubstType};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fun<Lang>
where
    Lang: Language,
{
    pub from: Box<Lang::Type>,
    pub to: Box<Lang::Type>,
}

impl<Lang> Fun<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1, Ty2>(from: Ty1, to: Ty2) -> Fun<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Fun {
            from: Box::new(from.into()),
            to: Box::new(to.into()),
        }
    }
}

impl<Lang> Type for Fun<Lang> where Lang: Language {}

impl<Lang> SubstType for Fun<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Fun {
            from: Box::new(self.from.subst_type(v, ty)),
            to: Box::new(self.to.subst_type(v, ty)),
        }
    }
}

impl<Lang> fmt::Display for Fun<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} -> {})", self.from, self.to)
    }
}
