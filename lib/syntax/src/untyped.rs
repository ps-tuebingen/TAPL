use crate::{
    TypeVar,
    language::Language,
    subst::SubstType,
    types::{Type, TypeGroup},
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Untyped<Lang>
where
    Lang: Language,
{
    phantom: PhantomData<Lang::Term>,
}

impl<Lang> Untyped<Lang>
where
    Lang: Language,
{
    pub fn new() -> Untyped<Lang> {
        Untyped {
            phantom: PhantomData,
        }
    }
}

impl<Lang> Type for Untyped<Lang> where Lang: Language {}

impl<Lang> fmt::Display for Untyped<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("")
    }
}

impl<Lang> SubstType for Untyped<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        self
    }
}

impl<Lang> TypeGroup for Untyped<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
}
