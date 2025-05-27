use super::Value;
use std::{fmt, marker::PhantomData};
use syntax::terms::{Something as SomethingT, Term};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Something<T, V>
where
    T: Term,
    V: Value,
{
    pub val: Box<V>,
    phantom: PhantomData<T>,
}

impl<T, V> Something<T, V>
where
    T: Term,
    V: Value,
{
    pub fn new<V1>(v: V1) -> Something<T, V>
    where
        V1: Into<V>,
    {
        Something {
            val: Box::new(v.into()),
            phantom: PhantomData,
        }
    }
}

impl<T, V> Value for Something<T, V>
where
    T: Term,
    V: Value,
{
    type Term = SomethingT<T>;
}

impl<T, V> From<Something<T, V>> for SomethingT<T>
where
    T: Term,
{
    fn from(something: Something<T, V>) -> SomethingT<T> {
        SomethingT::new(*something.val)
    }
}

impl<T, V> fmt::Display for Something<T, V>
where
    T: Term,
    V: Value,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something({})", self.val)
    }
}
