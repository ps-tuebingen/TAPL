use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Something<Lang>
where
    Lang: Language,
{
    pub term: Rc<Lang::Term>,
}

impl<Lang> Something<Lang>
where
    Lang: Language,
{
    pub fn new<T1>(t: T1) -> Something<Lang>
    where
        T1: Into<Lang::Term>,
    {
        Something {
            term: Rc::new(t.into()),
        }
    }
}

impl<Lang> Term for Something<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Something<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Something {
            term: self.term.subst(v, t),
        }
    }
}

impl<Lang> SubstType for Something<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Something {
            term: self.term.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Something<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something({})", self.term)
    }
}
