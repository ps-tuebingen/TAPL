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
    pub fn new() -> True<Lang> {
        True {
            phantom: PhantomData,
        }
    }
}

impl<Lang> Default for True<Lang>
where
    Lang: Language,
{
    fn default() -> True<Lang> {
        True::new()
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
    fn from(_: True<Lang>) -> TrueT<Lang> {
        TrueT::new()
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
