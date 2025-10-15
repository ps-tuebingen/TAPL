use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Fold<Lang>
where
    Lang: Language,
{
    pub term: Box<Lang::Term>,
    pub ty: Lang::Type,
}

impl<Lang> Fold<Lang>
where
    Lang: Language,
{
    pub fn new<T1, Typ>(t: T1, ty: Typ) -> Fold<Lang>
    where
        T1: Into<Lang::Term>,
        Typ: Into<Lang::Type>,
    {
        Fold {
            term: Box::new(t.into()),
            ty: ty.into(),
        }
    }
}

impl<Lang> Term for Fold<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Fold<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Fold {
            term: Box::new(self.term.subst(v, t)),
            ty: self.ty,
        }
    }
}

impl<Lang> SubstType for Fold<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Fold {
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Fold<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fold[{}]({})", self.ty, self.term)
    }
}
