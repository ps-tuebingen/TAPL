use super::Value;
use crate::{language::LanguageTerm, terms::Tuple as TupleT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tuple<T>
where
    T: LanguageTerm,
{
    pub vals: Vec<<T as LanguageTerm>::Value>,
}

impl<T> Tuple<T>
where
    T: LanguageTerm,
{
    pub fn new<V>(vals: Vec<V>) -> Tuple<T>
    where
        V: Into<<T as LanguageTerm>::Value>,
    {
        Tuple {
            vals: vals.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl<T> Value for Tuple<T>
where
    T: LanguageTerm,
{
    type Term = TupleT<T>;
}

impl<T> From<Tuple<T>> for TupleT<T>
where
    T: LanguageTerm,
{
    fn from(tup: Tuple<T>) -> TupleT<T> {
        TupleT::new(tup.vals)
    }
}

impl<T> fmt::Display for Tuple<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ts: Vec<String> = self.vals.iter().map(|t| t.to_string()).collect();
        ts.sort();
        write!(f, "{{ {} }}", ts.join(", "))
    }
}
