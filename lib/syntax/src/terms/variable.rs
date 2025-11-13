use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variable<Lang>
where
    Lang: Language,
{
    pub var: Var,
    phantom: PhantomData<Lang>,
}

impl<Lang> Variable<Lang>
where
    Lang: Language,
{
    #[must_use] pub fn new(v: &str) -> Self {
        Self {
            var: v.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<Lang> Term for Variable<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Variable<Lang>
where
    Lang: Language,
    Self: Into<Lang::Term>,
{
    type Target = Lang::Term;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        if *v == self.var {
            t.clone()
        } else {
            self.into()
        }
    }
}

impl<Lang> SubstType for Variable<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, _: &TypeVar, _: &<Lang as Language>::Type) -> Self::Target {
        self
    }
}

impl<Lang> fmt::Display for Variable<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.var)
    }
}
