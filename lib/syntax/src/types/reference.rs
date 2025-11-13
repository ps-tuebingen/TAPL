use super::Type;
use crate::{TypeVar, language::Language, subst::SubstType};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reference<Lang>
where
    Lang: Language,
{
    pub ty: Rc<Lang::Type>,
}

impl<Lang> Reference<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1>(ty: Ty1) -> Self
    where
        Ty1: Into<Lang::Type>,
    {
        Self {
            ty: Rc::new(ty.into()),
        }
    }
}

impl<Lang> Type for Reference<Lang> where Lang: Language {}

impl<Lang> SubstType for Reference<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            ty: self.ty.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Reference<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ref[{}]", self.ty)
    }
}
