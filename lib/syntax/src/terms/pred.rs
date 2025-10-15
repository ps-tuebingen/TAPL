use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pred<Lang>
where
    Lang: Language,
{
    pub term: Box<Lang::Term>,
}

impl<Lang> Pred<Lang>
where
    Lang: Language,
{
    pub fn new<T1>(t: T1) -> Pred<Lang>
    where
        T1: Into<Lang::Term>,
    {
        Pred {
            term: Box::new(t.into()),
        }
    }
}

impl<Lang> Term for Pred<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Pred<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Pred {
            term: Box::new(self.term.subst(v, t)),
        }
    }
}

impl<Lang> SubstType for Pred<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Pred {
            term: Box::new(self.term.subst_type(v, ty)),
        }
    }
}

impl<Lang> fmt::Display for Pred<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pred({})", self.term)
    }
}
