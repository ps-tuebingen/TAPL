use super::Value;
use crate::terms::{Term, Unit as UnitT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Unit;

impl<T> Value<T> for Unit
where
    T: Term + From<UnitT<T>>,
{
    type Term = UnitT<T>;
}

impl<T> From<Unit> for UnitT<T>
where
    T: Term + From<UnitT<T>>,
{
    fn from(_: Unit) -> UnitT<T> {
        UnitT::new()
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("unit")
    }
}
