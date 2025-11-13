use super::Value;
use crate::{language::Language, terms::Pack as PackT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pack<Lang>
where
    Lang: Language,
{
    pub inner_ty: Lang::Type,
    pub val: Box<Lang::Value>,
    pub outer_ty: Lang::Type,
}

impl<Lang> Pack<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1, V1, Ty2>(inner: Ty1, v: V1, outer: Ty2) -> Self
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
        V1: Into<Lang::Value>,
    {
        Self {
            inner_ty: inner.into(),
            val: Box::new(v.into()),
            outer_ty: outer.into(),
        }
    }
}

impl<Lang> Value for Pack<Lang>
where
    Lang: Language,
    PackT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = PackT<Lang>;
}

impl<Lang> From<Pack<Lang>> for PackT<Lang>
where
    Lang: Language,
{
    fn from(pack: Pack<Lang>) -> Self {
        Self::new(pack.inner_ty, *pack.val, pack.outer_ty)
    }
}

impl<Lang> fmt::Display for Pack<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{*{},{}}} as {}",
            self.inner_ty, self.val, self.outer_ty
        )
    }
}
