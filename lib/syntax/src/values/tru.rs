use super::Value;
use crate::{language::Language, terms::True as TrueT};
use std::{fmt, marker::PhantomData};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct True<Lang>
where
    Lang: Language,
{
    phantom: PhantomData<Lang>,
}

impl<Lang> True<Lang>
where
    Lang: Language,
{
    #[must_use] pub const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<Lang> Default for True<Lang>
where
    Lang: Language,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Lang> Value for True<Lang>
where
    Lang: Language,
    TrueT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = TrueT<Lang>;
}

impl<Lang> From<True<Lang>> for TrueT<Lang>
where
    Lang: Language,
{
    fn from(_: True<Lang>) -> Self {
        Self::new()
    }
}

impl<Lang> fmt::Display for True<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("true")
    }
}
