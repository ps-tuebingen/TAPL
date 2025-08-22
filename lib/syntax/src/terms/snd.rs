use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snd<Lang>
where
    Lang: Language,
{
    pub term: Box<Lang::Term>,
}

impl<Lang> Snd<Lang>
where
    Lang: Language,
{
    pub fn new<T1>(t: T1) -> Snd<Lang>
    where
        T1: Into<Lang::Term>,
    {
        Snd {
            term: Box::new(t.into()),
        }
    }
}

impl<Lang> Term for Snd<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Snd<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Snd {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<Lang> SubstType for Snd<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Snd {
            term: Box::new(self.term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Lang> fmt::Display for Snd<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}).snd", self.term)
    }
}
