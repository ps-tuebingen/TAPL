use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unfold<Lang>
where
    Lang: Language,
{
    pub ty: Lang::Type,
    pub term: Box<Lang::Term>,
}

impl<Lang> Unfold<Lang>
where
    Lang: Language,
{
    pub fn new<T1, Ty1>(ty: Ty1, t: T1) -> Unfold<Lang>
    where
        T1: Into<Lang::Term>,
        Ty1: Into<Lang::Type>,
    {
        Unfold {
            ty: ty.into(),
            term: Box::new(t.into()),
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
        Unfold {
            ty: self.ty,
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<Lang> SubstType for Unfold<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Unfold {
            ty: self.ty.subst_type(v, ty),
            term: Box::new(self.term.subst_type(v, ty)),
        }
        .into()
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
