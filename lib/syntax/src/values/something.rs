use super::Value;
use crate::{language::Language, terms::Something as SomethingT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Something<Lang>
where
    Lang: Language,
{
    pub val: Box<Lang::Value>,
}

impl<Lang> Something<Lang>
where
    Lang: Language,
{
    pub fn new<V1>(v: V1) -> Self
    where
        V1: Into<Lang::Value>,
    {
        Self {
            val: Box::new(v.into()),
        }
    }
}

impl<Lang> Value for Something<Lang>
where
    Lang: Language,
    SomethingT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = SomethingT<Lang>;
}

impl<Lang> From<Something<Lang>> for SomethingT<Lang>
where
    Lang: Language,
{
    fn from(something: Something<Lang>) -> Self {
        Self::new(*something.val)
    }
}

impl<Lang> fmt::Display for Something<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something({})", self.val)
    }
}
