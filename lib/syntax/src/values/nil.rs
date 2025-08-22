use super::Value;
use crate::{language::Language, terms::Nil as NilT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Nil<Lang>
where
    Lang: Language,
{
    pub ty: Lang::Type,
}

impl<Lang> Nil<Lang>
where
    Lang: Language,
{
    pub fn new<Ty>(ty: Ty) -> Nil<Lang>
    where
        Ty: Into<Lang::Type>,
    {
        Nil { ty: ty.into() }
    }
}

impl<Lang> Value for Nil<Lang>
where
    Lang: Language,
    NilT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = NilT<Lang>;
}

impl<Lang> From<Nil<Lang>> for NilT<Lang>
where
    Lang: Language,
{
    fn from(nil: Nil<Lang>) -> NilT<Lang> {
        NilT::new(nil.ty)
    }
}

impl<Lang> fmt::Display for Nil<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Nil[{}]", self.ty)
    }
}
