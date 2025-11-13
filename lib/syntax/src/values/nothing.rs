use super::Value;
use crate::{language::Language, terms::Nothing as NothingT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
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
    pub fn new<Ty>(ty: Ty) -> Self
    where
        Ty: Into<Lang::Type>,
    {
        Self { ty: ty.into() }
    }
}

impl<Lang> Value for Nothing<Lang>
where
    Lang: Language,
    NothingT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = NothingT<Lang>;
}

impl<Lang> From<Nothing<Lang>> for NothingT<Lang>
where
    Lang: Language,
{
    fn from(not: Nothing<Lang>) -> Self {
        Self::new(not.ty)
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
