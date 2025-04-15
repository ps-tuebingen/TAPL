use super::Value;
use crate::terms::{Term, Tuple as TupleT};
use std::fmt;
use std::marker::PhantomData;
#[derive(Debug)]
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
    T: Term + From<TupleT<T>>,
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

impl<V, T> fmt::Display for Tuple<V, T>
where
    V: Value<T>,
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ts: Vec<String> = self.vals.iter().map(|t| t.to_string()).collect();
        ts.sort();
        write!(f, "( {} )", ts.join(", "))
    }
}
