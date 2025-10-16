use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fix<Lang>
where
    Lang: Language,
{
    pub term: Rc<Lang::Term>,
}

impl<Lang> Fix<Lang>
where
    Lang: Language,
{
    pub fn new<T1>(t: T1) -> Fix<Lang>
    where
        T1: Into<Lang::Term>,
    {
        Fix {
            term: Rc::new(t.into()),
        }
    }
}

impl<Lang> Term for Fix<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Fix<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Fix {
            term: self.term.subst(v, t),
        }
    }
}

impl<Lang> SubstType for Fix<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Fix {
            term: self.term.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Fix<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fix({})", self.term)
    }
}
