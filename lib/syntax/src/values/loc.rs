use super::Value;
use crate::{language::Language, terms::Loc as LocT};
use std::{fmt, marker::PhantomData};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Loc<Lang>
where
    Lang: Language,
{
    pub loc: usize,
    phantom: PhantomData<Lang>,
}

impl<Lang> Loc<Lang>
where
    Lang: Language,
{
    #[must_use] pub const fn new(loc: usize) -> Self {
        Self {
            loc,
            phantom: PhantomData,
        }
    }
}

impl<Lang> Value for Loc<Lang>
where
    Lang: Language,
    LocT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = LocT<Lang>;
}

impl<Lang> From<Loc<Lang>> for LocT<Lang>
where
    Lang: Language,
{
    fn from(loc: Loc<Lang>) -> Self {
        Self::new(loc.loc)
    }
}

impl<Lang> fmt::Display for Loc<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.loc)
    }
}
