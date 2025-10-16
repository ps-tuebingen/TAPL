use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ref<Lang>
where
    Lang: Language,
{
    pub term: Rc<Lang::Term>,
}

impl<Lang> Ref<Lang>
where
    Lang: Language,
{
    pub fn new<T1>(t: T1) -> Ref<Lang>
    where
        T1: Into<Lang::Term>,
    {
        Ref {
            term: Rc::new(t.into()),
        }
    }
}

impl<Lang> Term for Ref<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Ref<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Ref {
            term: self.term.subst(v, t),
        }
    }
}

impl<Lang> SubstType for Ref<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Ref {
            term: self.term.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Ref<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ref({})", self.term)
    }
}
