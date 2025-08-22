use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nothing<Lang>
where
    Lang: Language,
{
    pub ty: Lang::Type,
}

impl<Lang> Nothing<Lang>
where
    Lang: Language,
{
    pub fn new<Typ>(ty: Typ) -> Nothing<Lang>
    where
        Typ: Into<Lang::Type>,
    {
        Nothing { ty: ty.into() }
    }
}

impl<Lang> Term for Nothing<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Nothing<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, _: &Var, _: &<Lang as Language>::Term) -> Self::Target {
        self.into()
    }
}

impl<Lang> SubstType for Nothing<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Nothing {
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<Lang> fmt::Display for Nothing<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nothing[{}]", self.ty)
    }
}
