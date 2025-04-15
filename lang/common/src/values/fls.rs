use super::Value;
use crate::terms::{False as FalseT, Term};

pub struct False;

impl<T> Value<T> for False
where
    T: Term,
{
    type Term = FalseT<T>;
}

impl<T> From<False> for FalseT<T>
where
    T: Term,
{
    fn from(_: False) -> FalseT<T> {
        FalseT::new()
    }
}
