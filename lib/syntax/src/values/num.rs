use super::Value;
use crate::{language::Language, terms::Num as NumT};
use std::{fmt, marker::PhantomData};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Num<Lang>
where
    Lang: Language,
{
    pub num: i64,
    phantom: PhantomData<Lang>,
}

impl<Lang> Num<Lang>
where
    Lang: Language,
{
    #[must_use] pub const fn new(i: i64) -> Self {
        Self {
            num: i,
            phantom: PhantomData,
        }
    }
}

impl<Lang> Value for Num<Lang>
where
    Lang: Language,
    NumT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = NumT<Lang>;
}

impl<Lang> From<Num<Lang>> for NumT<Lang>
where
    Lang: Language,
{
    fn from(n: Num<Lang>) -> Self {
        Self::new(n.num)
    }
}

impl<Lang> fmt::Display for Num<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}
