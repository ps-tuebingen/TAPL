use super::Type;
use crate::{TypeVar, kinds::Kind, language::Language, subst::SubstType};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Top<Lang>
where
    Lang: Language,
{
    pub kind: Kind,
    phantom: PhantomData<Lang>,
}

impl<Lang> Top<Lang>
where
    Lang: Language,
{
    pub fn new(knd: Kind) -> Top<Lang> {
        Top {
            kind: knd,
            phantom: PhantomData,
        }
    }

    pub fn new_star() -> Top<Lang> {
        Top {
            kind: Kind::Star,
            phantom: PhantomData,
        }
    }
}

impl<Lang> Type for Top<Lang> where Lang: Language {}

impl<Lang> SubstType for Top<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        self
    }
}

impl<Lang> fmt::Display for Top<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Top[{}]", self.kind)
    }
}
