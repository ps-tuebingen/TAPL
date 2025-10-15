use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Succ<Lang>
where
    Lang: Language,
{
    pub term: Box<Lang::Term>,
}

impl<Lang> Succ<Lang>
where
    Lang: Language,
{
    pub fn new<T1>(t: T1) -> Succ<Lang>
    where
        T1: Into<Lang::Term>,
    {
        Succ {
            term: Box::new(t.into()),
        }
    }
}

impl<Lang> Term for Succ<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Succ<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Succ {
            term: Box::new(self.term.subst(v, t)),
        }
    }
}

impl<Lang> SubstType for Succ<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Succ {
            term: Box::new(self.term.subst_type(v, ty)),
        }
    }
}

impl<Lang> fmt::Display for Succ<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "succ({})", self.term)
    }
}
