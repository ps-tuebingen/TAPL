use super::Type;
use crate::{TypeVar, language::Language, subst::SubstType};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bool<Lang>
where
    Lang: Language,
{
    phantom: PhantomData<Lang>,
}

impl<Lang> Bool<Lang>
where
    Lang: Language,
{
    pub fn new() -> Bool<Lang> {
        Bool {
            phantom: PhantomData,
        }
    }
}

impl<Lang> Default for Bool<Lang>
where
    Lang: Language,
{
    fn default() -> Bool<Lang> {
        Bool::new()
    }
}

impl<Lang> Type for Bool<Lang> where Lang: Language {}

impl<Lang> SubstType for Bool<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        self
    }
}

impl<Lang> fmt::Display for Bool<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Bool")
    }
}
