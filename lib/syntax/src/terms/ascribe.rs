use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ascribe<Lang>
where
    Lang: Language,
{
    pub term: Box<Lang::Term>,
    pub ty: Lang::Type,
}

impl<Lang> Ascribe<Lang>
where
    Lang: Language,
{
    pub fn new<T1, Ty1>(t: T1, ty: Ty1) -> Ascribe<Lang>
    where
        T1: Into<Lang::Term>,
        Ty1: Into<Lang::Type>,
    {
        Ascribe {
            term: Box::new(t.into()),
            ty: ty.into(),
        }
    }
}

impl<Lang> Term for Ascribe<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Ascribe<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Ascribe {
            term: Box::new(self.term.subst(v, t)),
            ty: self.ty,
        }
    }
}

impl<Lang> SubstType for Ascribe<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Ascribe {
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Ascribe<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} : {})", self.term, self.ty)
    }
}
