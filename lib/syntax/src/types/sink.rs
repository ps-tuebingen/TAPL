use super::Type;
use crate::{TypeVar, language::Language, subst::SubstType};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sink<Lang>
where
    Lang: Language,
{
    pub ty: Rc<Lang::Type>,
}

impl<Lang> Sink<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1>(ty: Ty1) -> Sink<Lang>
    where
        Ty1: Into<Lang::Type>,
    {
        Sink {
            ty: Rc::new(ty.into()),
        }
    }
}

impl<Lang> Type for Sink<Lang> where Lang: Language {}

impl<Lang> SubstType for Sink<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Sink {
            ty: self.ty.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Sink<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sink[{}]", self.ty)
    }
}
