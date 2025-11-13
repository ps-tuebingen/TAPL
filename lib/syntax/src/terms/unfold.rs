use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unfold<Lang>
where
    Lang: Language,
{
    pub ty: Lang::Type,
    pub term: Rc<Lang::Term>,
}

impl<Lang> Unfold<Lang>
where
    Lang: Language,
{
    pub fn new<T1, Ty1>(ty: Ty1, t: T1) -> Self
    where
        T1: Into<Lang::Term>,
        Ty1: Into<Lang::Type>,
    {
        Self {
            ty: ty.into(),
            term: Rc::new(t.into()),
        }
    }
}

impl<Lang> Term for Unfold<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Unfold<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Self {
            ty: self.ty,
            term: self.term.subst(v, t),
        }
    }
}

impl<Lang> SubstType for Unfold<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            ty: self.ty.subst_type(v, ty),
            term: self.term.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Unfold<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unfold[{}]({})", self.ty, self.term)
    }
}
