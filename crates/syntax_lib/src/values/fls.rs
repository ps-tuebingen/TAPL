use super::Value;
use crate::terms::{False as FalseT, Term};
use common::errors::ValueKind;
use std::{fmt, marker::PhantomData};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct False<T>
where
    T: Term,
{
    phantom: PhantomData<T>,
}

impl<T> False<T>
where
    T: Term,
{
    pub fn new() -> False<T> {
        False {
            phantom: PhantomData,
        }
    }
}

impl<T> Default for False<T>
where
    T: Term,
{
    fn default() -> False<T> {
        False::new()
    }
}

impl<T> Value for False<T>
where
    T: Term,
{
    type Term = FalseT<T>;

    fn knd(&self) -> ValueKind {
        ValueKind::False
    }
}

impl<T> From<False<T>> for FalseT<T>
where
    T: Term,
{
    fn from(_: False<T>) -> FalseT<T> {
        FalseT::new()
    }
}

impl<T> fmt::Display for False<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("false")
    }
}
