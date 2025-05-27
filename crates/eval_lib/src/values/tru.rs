use super::Value;
use std::{fmt, marker::PhantomData};
use syntax::terms::{Term, True as TrueT};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct True<T>
where
    T: Term,
{
    phantom: PhantomData<T>,
}

impl<T> True<T>
where
    T: Term,
{
    pub fn new() -> True<T> {
        True {
            phantom: PhantomData,
        }
    }
}

impl<T> Default for True<T>
where
    T: Term,
{
    fn default() -> True<T> {
        True::new()
    }
}

impl<T> Value for True<T>
where
    T: Term,
{
    type Term = TrueT<T>;
}

impl<T> From<True<T>> for TrueT<T>
where
    T: Term,
{
    fn from(_: True<T>) -> TrueT<T> {
        TrueT::new()
    }
}

impl<T> fmt::Display for True<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("true")
    }
}
