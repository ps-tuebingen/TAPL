use super::Value;
use crate::{language::LanguageTerm, terms::True as TrueT};
use std::{fmt, marker::PhantomData};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct True<T>
where
    T: LanguageTerm,
{
    phantom: PhantomData<T>,
}

impl<T> True<T>
where
    T: LanguageTerm,
{
    pub fn new() -> True<T> {
        True {
            phantom: PhantomData,
        }
    }
}

impl<T> Value for True<T>
where
    T: LanguageTerm,
{
    type Term = TrueT<T>;
}

impl<T> From<True<T>> for TrueT<T>
where
    T: LanguageTerm,
{
    fn from(_: True<T>) -> TrueT<T> {
        TrueT::new()
    }
}

impl<T> fmt::Display for True<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("true")
    }
}
