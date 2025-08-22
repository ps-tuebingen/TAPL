use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Projection<Lang>
where
    Lang: Language,
{
    pub term: Box<Lang::Term>,
    pub index: usize,
}

impl<Lang> Projection<Lang>
where
    Lang: Language,
{
    pub fn new<T1>(t: T1, ind: usize) -> Projection<Lang>
    where
        T1: Into<Lang::Term>,
    {
        Projection {
            term: Box::new(t.into()),
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
        Projection {
            term: Box::new(self.term.subst(v, t)),
            index: self.index,
        }
        .into()
    }
}

impl<Lang> SubstType for Projection<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Projection {
            term: Box::new(self.term.subst_type(v, ty)),
            index: self.index,
        }
        .into()
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
