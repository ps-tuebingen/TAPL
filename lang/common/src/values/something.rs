use super::Value;
use crate::terms::{Something as SomethingT, Term};
use std::fmt;
use std::marker::PhantomData;
#[derive(Debug, Clone)]
pub struct Something<V, T>
where
    V: Value<T>,
    T: Term,
{
    val: Box<V>,
    phantom: PhantomData<T>,
}

impl<V, T> Value<T> for Something<V, T>
where
    V: Value<T> + Into<T>,
    T: Term + From<SomethingT<T>>,
{
    type Term = SomethingT<T>;
}

impl<V, T> From<Something<V, T>> for SomethingT<T>
where
    T: Term,
    V: Value<T> + Into<T>,
{
    fn from(something: Something<V, T>) -> SomethingT<T> {
        SomethingT::new(*something.val)
    }
}

impl<V, T> fmt::Display for Something<V, T>
where
    V: Value<T>,
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something({})", self.val)
    }
}
