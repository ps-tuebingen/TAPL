use super::Value;
use crate::{language::LanguageTerm, terms::Loc as LocT};
use std::{fmt, marker::PhantomData};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Loc<T>
where
    T: LanguageTerm,
{
    pub loc: usize,
    phantom: PhantomData<T>,
}

impl<T> Loc<T>
where
    T: LanguageTerm,
{
    pub fn new(loc: usize) -> Loc<T> {
        Loc {
            loc,
            phantom: PhantomData,
        }
    }
}

impl<T> Value for Loc<T>
where
    T: LanguageTerm,
{
    type Term = LocT<T>;
}

impl<T> From<Loc<T>> for LocT<T>
where
    T: LanguageTerm,
{
    fn from(loc: Loc<T>) -> LocT<T> {
        LocT::new(loc.loc)
    }
}

impl<T> fmt::Display for Loc<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.loc)
    }
}
