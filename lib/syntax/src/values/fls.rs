use super::Value;
use crate::{language::Language, terms::False as FalseT};
use std::{fmt, marker::PhantomData};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct False<Lang>
where
    Lang: Language,
{
    phantom: PhantomData<Lang>,
}

impl<Lang> False<Lang>
where
    Lang: Language,
{
    pub fn new() -> False<Lang> {
        False {
            phantom: PhantomData,
        }
    }
}

impl<Lang> Default for False<Lang>
where
    Lang: Language,
{
    fn default() -> False<Lang> {
        False::new()
    }
}

impl<Lang> Value for False<Lang>
where
    Lang: Language,
    FalseT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = FalseT<Lang>;
}

impl<Lang> From<False<Lang>> for FalseT<Lang>
where
    Lang: Language,
{
    fn from(_: False<Lang>) -> FalseT<Lang> {
        FalseT::new()
    }
}

impl<Lang> fmt::Display for False<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("false")
    }
}
