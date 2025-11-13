use super::Value;
use crate::{language::Language, terms::Left as LeftT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Left<Lang>
where
    Lang: Language,
{
    pub left_val: Box<Lang::Value>,
    pub ty: Lang::Type,
}

impl<Lang> Left<Lang>
where
    Lang: Language,
{
    pub fn new<V1, Ty1>(val: V1, ty: Ty1) -> Self
    where
        V1: Into<Lang::Value>,
        Ty1: Into<Lang::Type>,
    {
        Self {
            left_val: Box::new(val.into()),
            ty: ty.into(),
        }
    }
}

impl<Lang> Value for Left<Lang>
where
    Lang: Language,
    LeftT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = LeftT<Lang>;
}

impl<Lang> From<Left<Lang>> for LeftT<Lang>
where
    Lang: Language,
{
    fn from(lft: Left<Lang>) -> Self {
        Self::new(*lft.left_val, lft.ty)
    }
}

impl<Lang> fmt::Display for Left<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inl({}) as {}", self.left_val, self.ty)
    }
}
