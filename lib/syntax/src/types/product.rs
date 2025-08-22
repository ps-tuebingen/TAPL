use super::Type;
use crate::{TypeVar, language::Language, subst::SubstType};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Product<Lang>
where
    Lang: Language,
{
    pub fst: Box<Lang::Type>,
    pub snd: Box<Lang::Type>,
}

impl<Lang> Product<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1, Ty2>(fst: Ty1, snd: Ty2) -> Product<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Product {
            fst: Box::new(fst.into()),
            snd: Box::new(snd.into()),
        }
    }
}

impl<Lang> Type for Product<Lang> where Lang: Language {}

impl<Lang> SubstType for Product<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Target = Self;
    type Lang = Lang;

    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Product {
            fst: Box::new(self.fst.subst_type(v, ty)),
            snd: Box::new(self.snd.subst_type(v, ty)),
        }
    }
}

impl<Lang> fmt::Display for Product<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} x {})", self.fst, self.snd)
    }
}
