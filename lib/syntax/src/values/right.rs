use super::Value;
use crate::{language::Language, terms::Right as RightT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Right<Lang>
where
    Lang: Language,
{
    pub right_val: Box<Lang::Value>,
    pub ty: Lang::Type,
}

impl<Lang> Right<Lang>
where
    Lang: Language,
{
    pub fn new<V1, Ty1>(val: V1, ty: Ty1) -> Self
    where
        V1: Into<Lang::Value>,
        Ty1: Into<Lang::Type>,
    {
        Self {
            right_val: Box::new(val.into()),
            ty: ty.into(),
        }
    }
}

impl<Lang> Value for Right<Lang>
where
    Lang: Language,
    RightT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = RightT<Lang>;
}

impl<Lang> From<Right<Lang>> for RightT<Lang>
where
    Lang: Language,
{
    fn from(right: Right<Lang>) -> Self {
        Self::new(*right.right_val, right.ty)
    }
}

impl<Lang> fmt::Display for Right<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inr({}) as {}", self.right_val, self.ty)
    }
}
