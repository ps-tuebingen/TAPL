use super::Value;
use crate::{
    errors::ErrorKind,
    terms::{Term, True as TrueT},
};
use std::fmt;

#[derive(Debug, Clone)]
pub struct True;

impl<T> Value<T> for True
where
    T: Term + From<TrueT<T>>,
{
    type Term = TrueT<T>;

    fn into_true(self) -> Result<True, ErrorKind> {
        Ok(self)
    }
}

impl<T> From<True> for TrueT<T>
where
    T: Term + From<TrueT<T>>,
{
    fn from(_: True) -> TrueT<T> {
        TrueT::new()
    }
}

impl fmt::Display for True {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("true")
    }
}
