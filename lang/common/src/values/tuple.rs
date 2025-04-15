use super::Value;
use crate::terms::{Term, Tuple as TupleT};
use std::marker::PhantomData;

pub struct Tuple<V, T>
where
    V: Value<T>,
    T: Term,
{
    vals: Vec<V>,
    phantom: PhantomData<T>,
}

impl<V, T> Value<T> for Tuple<V, T>
where
    V: Value<T> + Into<T>,
    T: Term,
{
    type Term = TupleT<T>;
}

impl<V, T> From<Tuple<V, T>> for TupleT<T>
where
    V: Value<T> + Into<T>,
    T: Term,
{
    fn from(tup: Tuple<V, T>) -> TupleT<T> {
        TupleT::new(tup.vals)
    }
}
