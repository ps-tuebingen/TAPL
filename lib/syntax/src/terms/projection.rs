use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Projection<Lang>
where
    Lang: Language,
{
    pub term: Rc<Lang::Term>,
    pub index: usize,
}

impl<Lang> Projection<Lang>
where
    Lang: Language,
{
    pub fn new<T1>(t: T1, ind: usize) -> Self
    where
        T1: Into<Lang::Term>,
    {
        Self {
            term: Rc::new(t.into()),
            index: ind,
        }
    }
}

impl<Lang> Term for Projection<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Projection<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Self {
            term: self.term.subst(v, t),
            index: self.index,
        }
    }
}

impl<Lang> SubstType for Projection<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            term: self.term.subst_type(v, ty),
            index: self.index,
        }
    }
}

impl<Lang> fmt::Display for Projection<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}).{}", self.term, self.index)
    }
}
