use super::Type;
use crate::{TypeVar, language::Language, subst::SubstType};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
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
    pub fn new() -> Unit<Lang> {
        Unit {
            phantom: PhantomData,
        }
    }
}

impl<Lang> Default for Unit<Lang>
where
    Lang: Language,
{
    fn default() -> Unit<Lang> {
        Unit::new()
    }
}

impl<Lang> Type for Unit<Lang> where Lang: Language {}

impl<Lang> SubstType for Unit<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        self
    }
}

impl<Lang> fmt::Display for Unit<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Unit")
    }
}
