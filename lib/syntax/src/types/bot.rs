use crate::{TypeVar, kinds::Kind, language::Language, subst::SubstType, types::Type};
use std::{fmt, marker::PhantomData};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bot<Lang>
where
    Lang: Language,
{
    pub kind: Kind,
    phantom: PhantomData<Lang>,
}

impl<Lang> Bot<Lang>
where
    Lang: Language,
{
    pub fn new() -> Bot<Lang> {
        Bot {
            kind: Kind::Star,
            phantom: PhantomData,
        }
    }
}

impl<Lang> Default for Bot<Lang>
where
    Lang: Language,
{
    fn default() -> Bot<Lang> {
        Bot::new()
    }
}

impl<Lang> Type for Bot<Lang> where Lang: Language {}

impl<Lang> SubstType for Bot<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        self
    }
}

impl<Lang> fmt::Display for Bot<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Bot")
    }
}
