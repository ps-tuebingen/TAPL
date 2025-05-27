use super::Value;
use std::{fmt, marker::PhantomData};
use syntax::terms::{Term, Tuple as TupleT};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tuple<T, V>
where
    T: Term,
    V: Value,
{
    pub vals: Vec<V>,
    phantom: PhantomData<T>,
}

impl<T, V> Tuple<T, V>
where
    T: Term,
    V: Value,
{
    pub fn new<V1>(vals: Vec<V1>) -> Tuple<T, V>
    where
        V1: Into<V>,
    {
        Tuple {
            vals: vals.into_iter().map(|v| v.into()).collect(),
            phantom: PhantomData,
        }
    }
}

impl<T, V> Value for Tuple<T, V>
where
    T: Term,
    V: Value,
{
    type Term = TupleT<T>;
}

impl<T, V> From<Tuple<T, V>> for TupleT<T>
where
    T: Term,
    V: Value,
{
    fn from(tup: Tuple<T, V>) -> TupleT<T> {
        TupleT::new(tup.vals)
    }
}

impl<T, V> fmt::Display for Tuple<T, V>
where
    T: Term,
    V: Value,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ts: Vec<String> = self.vals.iter().map(|t| t.to_string()).collect();
        ts.sort();
        write!(f, "{{ {} }}", ts.join(", "))
    }
}
