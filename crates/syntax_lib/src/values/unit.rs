use super::Value;
use crate::terms::{Term, Unit as UnitT};
use common::errors::ValueKind;
use std::{fmt, marker::PhantomData};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Unit<T>
where
    T: Term,
{
    phantom: PhantomData<T>,
}

impl<T> Unit<T>
where
    T: Term,
{
    pub fn new() -> Unit<T> {
        Unit {
            phantom: PhantomData,
        }
    }
}

impl<T> Default for Unit<T>
where
    T: Term,
{
    fn default() -> Unit<T> {
        Unit::new()
    }
}

impl<T> Value for Unit<T>
where
    T: Term,
{
    type Term = UnitT<T>;

    fn knd(&self) -> ValueKind {
        ValueKind::Unit
    }
}

impl<T> From<Unit<T>> for UnitT<T>
where
    T: Term,
{
    fn from(_: Unit<T>) -> UnitT<T> {
        UnitT::new()
    }
}

impl<T> fmt::Display for Unit<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("unit")
    }
}
