use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TryWithVal<Lang>
where
    Lang: Language,
{
    pub term: Box<Lang::Term>,
    pub handler: Box<Lang::Term>,
}

impl<Lang> TryWithVal<Lang>
where
    Lang: Language,
{
    pub fn new<T1, T2>(t: T1, h: T2) -> TryWithVal<Lang>
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        TryWithVal {
            term: Box::new(t.into()),
            handler: Box::new(h.into()),
        }
    }
}

impl<Lang> Term for TryWithVal<Lang> where Lang: Language {}

impl<Lang> SubstTerm for TryWithVal<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        TryWithVal {
            term: Box::new(self.term.subst(v, t)),
            handler: Box::new(self.handler.subst(v, t)),
        }
        .into()
    }
}

impl<Lang> SubstType for TryWithVal<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        TryWithVal {
            term: Box::new(self.term.subst_type(v, ty)),
            handler: Box::new(self.handler.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Lang> fmt::Display for TryWithVal<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "try {{ {} }} catch {{ {} }}", self.term, self.handler)
    }
}
