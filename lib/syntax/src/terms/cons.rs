use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cons<Lang>
where
    Lang: Language,
{
    pub head: Box<Lang::Term>,
    pub tail: Box<Lang::Term>,
    pub ty: Lang::Type,
}

impl<Lang> Cons<Lang>
where
    Lang: Language,
{
    pub fn new<H, Tl, Typ>(h: H, tl: Tl, ty: Typ) -> Cons<Lang>
    where
        H: Into<Lang::Term>,
        Tl: Into<Lang::Term>,
        Typ: Into<Lang::Type>,
    {
        Cons {
            head: Box::new(h.into()),
            tail: Box::new(tl.into()),
            ty: ty.into(),
        }
    }
}

impl<Lang> Term for Cons<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Cons<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Cons {
            head: Box::new(self.head.subst(v, t)),
            tail: Box::new(self.tail.subst(v, t)),
            ty: self.ty,
        }
    }
}

impl<Lang> SubstType for Cons<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Cons {
            head: Box::new(self.head.subst_type(v, ty)),
            tail: Box::new(self.tail.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Cons<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cons[{}]({},{})", self.ty, self.head, self.tail)
    }
}
