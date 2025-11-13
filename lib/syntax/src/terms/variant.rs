use super::Term;
use crate::{
    Label, TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, rc::Rc};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant<Lang>
where
    Lang: Language,
{
    pub label: Label,
    pub term: Rc<Lang::Term>,
    pub ty: Lang::Type,
}

impl<Lang> Variant<Lang>
where
    Lang: Language,
{
    pub fn new<T1, Ty1>(lb: &str, t: T1, ty: Ty1) -> Self
    where
        T1: Into<Lang::Term>,
        Ty1: Into<Lang::Type>,
    {
        Self {
            label: lb.to_owned(),
            term: Rc::new(t.into()),
            ty: ty.into(),
        }
    }
}

impl<Lang> Term for Variant<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Variant<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Self {
            label: self.label,
            term: self.term.subst(v, t),
            ty: self.ty,
        }
    }
}

impl<Lang> SubstType for Variant<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            label: self.label,
            term: self.term.subst_type(v, ty),
            ty: self.ty.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Variant<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}={}> as {}", self.label, self.term, self.ty)
    }
}
