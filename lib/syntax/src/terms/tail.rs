use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tail<Lang>
where
    Lang: Language,
{
    pub term: Rc<Lang::Term>,
    pub ty: Lang::Type,
}

impl<Lang> Tail<Lang>
where
    Lang: Language,
{
    pub fn new<T1, Ty1>(t: T1, ty: Ty1) -> Self
    where
        T1: Into<Lang::Term>,
        Ty1: Into<Lang::Type>,
    {
        Self {
            term: Rc::new(t.into()),
            ty: ty.into(),
        }
    }
}

impl<Lang> Term for Tail<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Tail<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Self {
            term: self.term.subst(v, t),
            ty: self.ty,
        }
    }
}

impl<Lang> SubstType for Tail<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            term: self.term.subst_type(v, ty),
            ty: self.ty.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Tail<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "tail[{}]({})", self.term, self.term)
    }
}
