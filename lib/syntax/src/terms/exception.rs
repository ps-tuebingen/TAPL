use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exception<Lang>
where
    Lang: Language,
{
    pub ty: Lang::Type,
}

impl<Lang> Exception<Lang>
where
    Lang: Language,
{
    pub fn new<Typ>(ty: Typ) -> Self
    where
        Typ: Into<Lang::Type>,
    {
        Self { ty: ty.into() }
    }
}

impl<Lang> Term for Exception<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Exception<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, _: &Var, _: &Lang::Term) -> Self::Target {
        self
    }
}

impl<Lang> SubstType for Exception<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            ty: self.ty.subst_type(v, ty),
        }
    }
}

impl<Lang> fmt::Display for Exception<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error[{}]", self.ty)
    }
}
