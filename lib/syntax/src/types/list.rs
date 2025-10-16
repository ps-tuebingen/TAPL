use super::Type;
use crate::{TypeVar, language::Language, subst::SubstType};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct List<Lang>
where
    Lang: Language,
{
    pub ty: Rc<Lang::Type>,
}

impl<Lang> List<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1>(ty: Ty1) -> List<Lang>
    where
        Ty1: Into<Lang::Type>,
    {
        List {
            ty: Rc::new(ty.into()),
        }
    }
}

impl<Lang> Type for List<Lang> where Lang: Language {}

impl<Lang> SubstType for List<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        List {
            ty: self.ty.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for List<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "List[{}]", self.ty)
    }
}
