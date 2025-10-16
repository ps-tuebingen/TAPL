use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pair<Lang>
where
    Lang: Language,
{
    pub fst: Rc<Lang::Term>,
    pub snd: Rc<Lang::Term>,
}

impl<Lang> Pair<Lang>
where
    Lang: Language,
{
    pub fn new<T1, T2>(fst: T1, snd: T2) -> Pair<Lang>
    where
        T1: Into<Lang::Term>,
        T2: Into<Lang::Term>,
    {
        Pair {
            fst: Rc::new(fst.into()),
            snd: Rc::new(snd.into()),
        }
    }
}

impl<Lang> Term for Pair<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Pair<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Pair {
            fst: self.fst.subst(v, t),
            snd: self.snd.subst(v, t),
        }
    }
}

impl<Lang> SubstType for Pair<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Pair {
            fst: self.fst.subst_type(v, ty),
            snd: self.snd.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Pair<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {}, {} }}", self.fst, self.snd)
    }
}
