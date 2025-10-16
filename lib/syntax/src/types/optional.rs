use super::Type;
use crate::{TypeVar, language::Language, subst::SubstType};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Optional<Lang>
where
    Lang: Language,
{
    pub ty: Rc<Lang::Type>,
}

impl<Lang> Optional<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1>(ty: Ty1) -> Optional<Lang>
    where
        Ty1: Into<Lang::Type>,
    {
        Optional {
            ty: Rc::new(ty.into()),
        }
    }
}

impl<Lang> Type for Optional<Lang> where Lang: Language {}

impl<Lang> SubstType for Optional<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Optional {
            ty: self.ty.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Optional<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Optional[{}]", self.ty)
    }
}
