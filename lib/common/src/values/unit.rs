use super::Value;
use crate::{language::LanguageTerm, terms::Unit as UnitT};
use std::{fmt, marker::PhantomData};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Unit<T>
where
    T: LanguageTerm,
{
    phantom: PhantomData<T>,
}

impl<T> Unit<T>
where
    T: LanguageTerm,
{
    pub fn new() -> Unit<T> {
        Unit {
            phantom: PhantomData,
        }
    }
}

impl<T> Default for Unit<T>
where
    T: LanguageTerm,
{
    fn default() -> Unit<T> {
        Unit::new()
    }
}

impl<T> Value for Unit<T>
where
    T: LanguageTerm,
{
    type Term = UnitT<T>;
}

impl<T> From<Unit<T>> for UnitT<T>
where
    T: LanguageTerm,
{
    fn from(_: Unit<T>) -> UnitT<T> {
        UnitT::new()
    }
}

impl<T> fmt::Display for Unit<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("unit")
    }
}
