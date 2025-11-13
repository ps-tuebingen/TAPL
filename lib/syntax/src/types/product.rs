use super::Type;
use crate::{TypeVar, language::Language, subst::SubstType};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Product<Lang>
where
    Lang: Language,
{
    pub fst: Rc<Lang::Type>,
    pub snd: Rc<Lang::Type>,
}

impl<Lang> Product<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1, Ty2>(fst: Ty1, snd: Ty2) -> Self
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Self {
            fst: Rc::new(fst.into()),
            snd: Rc::new(snd.into()),
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
        Self {
            fst: self.fst.subst_type(v, ty),
            snd: self.snd.subst_type(v, ty),
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
