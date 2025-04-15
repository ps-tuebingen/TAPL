use super::Value;
use crate::terms::{Term, Unit as UnitT};

pub struct Unit;

impl<T> Value<T> for Unit
where
    T: Term,
{
    type Term = UnitT<T>;
}

impl<T> From<Unit> for UnitT<T>
where
    T: Term,
{
    fn from(_: Unit) -> UnitT<T> {
        UnitT::new()
    }
}
