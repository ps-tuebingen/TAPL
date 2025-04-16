use super::Value;
use crate::{language::LanguageTerm, terms::False as FalseT};
use std::{fmt, marker::PhantomData};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct False<T>
where
    T: LanguageTerm,
{
    phantom: PhantomData<T>,
}

impl<T> False<T>
where
    T: LanguageTerm,
{
    pub fn new() -> False<T> {
        False {
            phantom: PhantomData,
        }
    }
}

impl<T> Value for False<T>
where
    T: LanguageTerm,
{
    type Term = FalseT<T>;
}

impl<T> From<False<T>> for FalseT<T>
where
    T: LanguageTerm,
{
    fn from(_: False<T>) -> FalseT<T> {
        FalseT::new()
    }
}

impl<T> fmt::Display for False<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("false")
    }
}
