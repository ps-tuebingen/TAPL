use super::Value;
use crate::{
    errors::ErrorKind,
    terms::{False as FalseT, Term},
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
