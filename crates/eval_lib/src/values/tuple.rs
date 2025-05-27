use super::Value;
use std::fmt;
use syntax::terms::Tuple as TupleT;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tuple<V>
where
    V: Value,
{
    pub vals: Vec<V>,
}

impl<V> Tuple<V>
where
    V: Value,
{
    pub fn new<V1>(vals: Vec<V1>) -> Tuple<V>
    where
        V1: Into<V>,
    {
        Tuple {
            vals: vals.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl<V> Value for Tuple<V>
where
    V: Value,
{
    type Term = TupleT<<V as Value>::Term>;
}

impl<V> From<Tuple<V>> for TupleT<<V as Value>::Term>
where
    V: Value,
{
    fn from(tup: Tuple<V>) -> TupleT<<V as Value>::Term> {
        TupleT::new(tup.vals)
    }
}

impl<V> fmt::Display for Tuple<V>
where
    V: Value,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ts: Vec<String> = self.vals.iter().map(|t| t.to_string()).collect();
        ts.sort();
        write!(f, "{{ {} }}", ts.join(", "))
    }
}
