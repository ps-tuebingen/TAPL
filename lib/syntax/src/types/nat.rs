use super::Type;
use crate::{TypeVar, language::Language, subst::SubstType};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Nat<Lang>
where
    Lang: Language,
{
    phantom: PhantomData<Lang>,
}

impl<Lang> Nat<Lang>
where
    Lang: Language,
{
    pub fn new() -> Nat<Lang> {
        Nat {
            phantom: PhantomData,
        }
    }
}

impl<Lang> Default for Nat<Lang>
where
    Lang: Language,
{
    fn default() -> Nat<Lang> {
        Nat::new()
    }
}

impl<Lang> Type for Nat<Lang> where Lang: Language {}

impl<Lang> SubstType for Nat<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        self
    }
}

impl<Lang> fmt::Display for Nat<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Nat")
    }
}
