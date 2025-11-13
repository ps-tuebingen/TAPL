use super::Value;
use crate::{language::Language, terms::Unit as UnitT};
use std::{fmt, marker::PhantomData};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Unit<Lang>
where
    Lang: Language,
{
    phantom: PhantomData<Lang>,
}

impl<Lang> Unit<Lang>
where
    Lang: Language,
{
    #[must_use] pub const fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<Lang> Default for Unit<Lang>
where
    Lang: Language,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Lang> Value for Unit<Lang>
where
    Lang: Language,
    UnitT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = UnitT<Lang>;
}

impl<Lang> From<Unit<Lang>> for UnitT<Lang>
where
    Lang: Language,
{
    fn from(_: Unit<Lang>) -> Self {
        Self::new()
    }
}

impl<Lang> fmt::Display for Unit<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("unit")
    }
}
