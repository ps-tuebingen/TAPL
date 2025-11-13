use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Num<Lang>
where
    Lang: Language,
{
    pub num: i64,
    phantom: PhantomData<Lang>,
}

impl<Lang> Num<Lang>
where
    Lang: Language,
{
    #[must_use] pub const fn new(num: i64) -> Self {
        Self {
            num,
            phantom: PhantomData,
        }
    }
}

impl<Lang> Term for Num<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Num<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, _: &Var, _: &<Lang as Language>::Term) -> Self::Target {
        self
    }
}

impl<Lang> SubstType for Num<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        self
    }
}

impl<Lang> fmt::Display for Num<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}
