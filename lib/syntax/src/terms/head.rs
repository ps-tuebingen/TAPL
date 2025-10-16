use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Head<Lang>
where
    Lang: Language,
{
    pub term: Rc<Lang::Term>,
    pub ty: Lang::Type,
}

impl<Lang> Head<Lang>
where
    Lang: Language,
{
    pub fn new<T1, Typ>(t: T1, ty: Typ) -> Head<Lang>
    where
        T1: Into<Lang::Term>,
        Typ: Into<Lang::Type>,
    {
        Head {
            term: Rc::new(t.into()),
            ty: ty.into(),
        }
    }
}

impl<Lang> Term for Head<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Head<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Head {
            term: self.term.subst(v, t),
            ty: self.ty,
        }
    }
}

impl<Lang> SubstType for Head<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Head {
            term: self.term.subst_type(v, ty),
            ty: self.ty.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Head<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "head[{}]({})", self.ty, self.term)
    }
}
