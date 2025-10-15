use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cast<Lang>
where
    Lang: Language,
{
    pub term: Box<Lang::Term>,
    pub ty: Lang::Type,
}

impl<Lang> Cast<Lang>
where
    Lang: Language,
{
    pub fn new<T1, Typ>(t: T1, ty: Typ) -> Cast<Lang>
    where
        T1: Into<Lang::Term>,
        Typ: Into<Lang::Type>,
    {
        Cast {
            term: Box::new(t.into()),
            ty: ty.into(),
        }
    }
}

impl<Lang> Term for Cast<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Cast<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Cast {
            term: Box::new(self.term.subst(v, t)),
            ty: self.ty,
        }
    }
}

impl<Lang> SubstType for Cast<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Cast {
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Cast<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} as {} ", self.term, self.ty)
    }
}
