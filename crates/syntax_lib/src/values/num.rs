use super::Value;
use crate::errors::ValueKind;
use std::{fmt, marker::PhantomData};
use syntax::terms::{Num as NumT, Term};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Num<T>
where
    T: Term,
{
    pub num: i64,
    phantom: PhantomData<T>,
}

impl<T> Num<T>
where
    T: Term,
{
    pub fn new(i: i64) -> Num<T> {
        Num {
            num: i,
            phantom: PhantomData,
        }
    }
}

impl<T> Value for Num<T>
where
    T: Term,
{
    type Term = NumT<T>;

    fn knd(&self) -> ValueKind {
        ValueKind::Number
    }
}

impl<T> From<Num<T>> for NumT<T>
where
    T: Term,
{
    fn from(n: Num<T>) -> NumT<T> {
        NumT::new(n.num)
    }
}

impl<T> fmt::Display for Num<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.num)
    }
}
