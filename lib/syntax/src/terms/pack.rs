use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pack<Lang>
where
    Lang: Language,
{
    pub inner_ty: Lang::Type,
    pub term: Box<Lang::Term>,
    pub outer_ty: Lang::Type,
}

impl<Lang> Pack<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1, Ty2, T1>(inner: Ty1, t: T1, outer: Ty2) -> Pack<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
        T1: Into<Lang::Term>,
    {
        Pack {
            inner_ty: inner.into(),
            term: Box::new(t.into()),
            outer_ty: outer.into(),
        }
    }
}

impl<Lang> Term for Pack<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Pack<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Pack {
            inner_ty: self.inner_ty,
            term: Box::new(self.term.subst(v, t)),
            outer_ty: self.outer_ty,
        }
        .into()
    }
}

impl<Lang> SubstType for Pack<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Pack {
            inner_ty: self.inner_ty.subst_type(v, ty),
            term: Box::new(self.term.subst_type(v, ty)),
            outer_ty: self.outer_ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<Lang> fmt::Display for Pack<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{*({}),{}}} as {}",
            self.inner_ty, self.term, self.outer_ty
        )
    }
}
