use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Right<Lang>
where
    Lang: Language,
{
    pub right_term: Box<Lang::Term>,
    pub ty: Lang::Type,
}

impl<Lang> Right<Lang>
where
    Lang: Language,
{
    pub fn new<T1, Ty1>(right_t: T1, ty: Ty1) -> Right<Lang>
    where
        T1: Into<Lang::Term>,
        Ty1: Into<Lang::Type>,
    {
        Right {
            right_term: Box::new(right_t.into()),
            ty: ty.into(),
        }
    }
}

impl<Lang> Term for Right<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Right<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Right {
            right_term: Box::new(self.right_term.subst(v, t)),
            ty: self.ty,
        }
    }
}

impl<Lang> SubstType for Right<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Right {
            right_term: Box::new(self.right_term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Right<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inl({}) as {}", self.right_term, self.ty)
    }
}
