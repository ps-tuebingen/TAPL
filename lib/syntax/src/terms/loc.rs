use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Loc<Lang>
where
    Lang: Language,
{
    pub loc: usize,
    phantom: PhantomData<Lang>,
}

impl<Lang> Loc<Lang>
where
    Lang: Language,
{
    pub fn new(loc: usize) -> Loc<Lang> {
        Loc {
            loc,
            phantom: PhantomData,
        }
    }
}

impl<Lang> Term for Loc<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Loc<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, _: &Var, _: &<Lang as Language>::Term) -> Self::Target {
        self
    }
}

impl<Lang> SubstType for Loc<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        self
    }
}

impl<Lang> fmt::Display for Loc<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.loc)
    }
}
