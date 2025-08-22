use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Deref<Lang>
where
    Lang: Language,
{
    pub term: Box<Lang::Term>,
}

impl<Lang> Deref<Lang>
where
    Lang: Language,
{
    pub fn new<T1>(t: T1) -> Deref<Lang>
    where
        T1: Into<Lang::Term>,
    {
        Deref {
            term: Box::new(t.into()),
        }
    }
}

impl<Lang> Term for Deref<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Deref<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Deref {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<Lang> SubstType for Deref<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Deref {
            term: Box::new(self.term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Lang> fmt::Display for Deref<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "!{}", self.term)
    }
}
