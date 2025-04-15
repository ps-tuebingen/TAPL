use super::{Lambda, Raise, Value};
use crate::{
    errors::ErrorKind,
    terms::{Term, Unit as UnitT},
    types::Type,
};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Unit;

impl<T> Value<T> for Unit
where
    T: Term + From<UnitT<T>>,
{
    type Term = UnitT<T>;
    fn into_lambda<Ty1>(self) -> Result<Lambda<T, Ty1>, ErrorKind>
    where
        Ty1: Type,
    {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Lambda Abstraction".to_owned(),
        })
    }

    fn into_raise<Val, Ty1>(self) -> Result<Raise<Val, Ty1, T>, ErrorKind>
    where
        Val: Value<T>,
        Ty1: Type,
    {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Raise".to_owned(),
        })
    }
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
