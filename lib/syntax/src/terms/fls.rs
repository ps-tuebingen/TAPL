use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
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

impl<Lang> Term for False<Lang> where Lang: Language {}

impl<Lang> SubstTerm for False<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, _: &Var, _: &<Lang as Language>::Term) -> Self::Target {
        self.into()
    }
}

impl<Lang> SubstType for False<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        False {
            phantom: PhantomData,
        }
        .into()
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
