use super::Type;
use crate::{TypeVar, language::Language, subst::SubstType};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sum<Lang>
where
    Lang: Language,
{
    pub left: Rc<Lang::Type>,
    pub right: Rc<Lang::Type>,
}

impl<Lang> Sum<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1, Ty2>(l: Ty1, r: Ty2) -> Sum<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Sum {
            left: Rc::new(l.into()),
            right: Rc::new(r.into()),
        }
    }
}

impl<Lang> Type for Sum<Lang> where Lang: Language {}

impl<Lang> SubstType for Sum<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Sum {
            left: self.left.subst_type(v, ty),
            right: self.right.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Sum<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}+{})", self.left, self.right)
    }
}
