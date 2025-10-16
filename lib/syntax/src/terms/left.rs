use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Left<Lang>
where
    Lang: Language,
{
    pub left_term: Rc<Lang::Term>,
    pub ty: Lang::Type,
}

impl<Lang> Left<Lang>
where
    Lang: Language,
{
    pub fn new<L, Typ>(left_t: L, ty: Typ) -> Left<Lang>
    where
        L: Into<Lang::Term>,
        Typ: Into<Lang::Type>,
    {
        Left {
            left_term: Rc::new(left_t.into()),
            ty: ty.into(),
        }
    }
}

impl<Lang> Term for Left<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Left<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Left {
            left_term: self.left_term.subst(v, t),
            ty: self.ty,
        }
    }
}

impl<Lang> SubstType for Left<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Left {
            left_term: self.left_term.subst_type(v, ty),
            ty: self.ty.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Left<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inl({}) as {}", self.left_term, self.ty)
    }
}
