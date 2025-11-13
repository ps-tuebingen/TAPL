use super::Value;
use crate::{language::Language, terms::Fold as FoldT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Fold<Lang>
where
    Lang: Language,
{
    pub ty: Lang::Type,
    pub val: Box<Lang::Value>,
}

impl<Lang> Fold<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1, V1>(ty: Ty1, v: V1) -> Self
    where
        Ty1: Into<Lang::Type>,
        V1: Into<Lang::Value>,
    {
        Self {
            ty: ty.into(),
            val: Box::new(v.into()),
        }
    }
}

impl<Lang> Value for Fold<Lang>
where
    Lang: Language,
    FoldT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = FoldT<Lang>;
}

impl<Lang> From<Fold<Lang>> for FoldT<Lang>
where
    Lang: Language,
{
    fn from(fld: Fold<Lang>) -> Self {
        Self::new(*fld.val, fld.ty)
    }
}

impl<Lang> fmt::Display for Fold<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fold[{}]({})", self.ty, self.val)
    }
}
