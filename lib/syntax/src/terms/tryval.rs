use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TryWithVal<Lang>
where
    Lang: Language,
{
    pub term: Rc<Lang::Term>,
    pub handler: Rc<Lang::Term>,
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
            term: Rc::new(t.into()),
            handler: Rc::new(h.into()),
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
            term: self.term.subst(v, t),
            handler: self.handler.subst(v, t),
        }
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
            term: self.term.subst_type(v, ty),
            handler: self.handler.subst_type(v, ty),
        }
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
