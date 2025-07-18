use super::Value;
use crate::terms::{Loc as LocT, Term};
use errors::ValueKind;
use std::{fmt, marker::PhantomData};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Loc<T>
where
    T: Term,
{
    pub loc: usize,
    phantom: PhantomData<T>,
}

impl<T> Loc<T>
where
    T: Term,
{
    pub fn new(loc: usize) -> Loc<T> {
        Loc {
            loc,
            phantom: PhantomData,
        }
    }
}

impl<T> Value for Loc<T>
where
    T: Term,
{
    type Term = LocT<T>;

    fn knd(&self) -> ValueKind {
        ValueKind::Location
    }
}

impl<T> From<Loc<T>> for LocT<T>
where
    T: Term,
{
    fn from(loc: Loc<T>) -> LocT<T> {
        LocT::new(loc.loc)
    }
}

impl<T> fmt::Display for Loc<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.loc)
    }
}
