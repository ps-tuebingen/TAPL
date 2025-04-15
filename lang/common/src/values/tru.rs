use super::Value;
use crate::terms::{Term, True as TrueT};
use std::fmt;

#[derive(Debug)]
pub struct True;

impl<T> Value<T> for True
where
    T: Term + From<TrueT<T>>,
{
    type Term = TrueT<T>;
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
