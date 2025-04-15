use super::{Lambda, Raise, Value};
use crate::{
    errors::ErrorKind,
    terms::{False as FalseT, Term},
    types::Type,
};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct False;

impl<T> Value<T> for False
where
    T: Term + From<FalseT<T>>,
{
    type Term = FalseT<T>;

    fn into_false(self) -> Result<False, ErrorKind> {
        Ok(self)
    }
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

impl<T> From<False> for FalseT<T>
where
    T: Term,
{
    fn from(_: False) -> FalseT<T> {
        FalseT::new()
    }
}

impl fmt::Display for False {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("false")
    }
}
