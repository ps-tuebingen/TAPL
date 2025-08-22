use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tail<Lang>
where
    Lang: Language,
{
    pub term: Box<Lang::Term>,
    pub ty: Lang::Type,
}

impl<Lang> Tail<Lang>
where
    Lang: Language,
{
    pub fn new<T1, Ty1>(t: T1, ty: Ty1) -> Tail<Lang>
    where
        T1: Into<Lang::Term>,
        Ty1: Into<Lang::Type>,
    {
        Tail {
            term: Box::new(t.into()),
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
        Tail {
            term: Box::new(self.term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<Lang> SubstType for Tail<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Tail {
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
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
