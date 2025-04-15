use super::{Lambda, Raise, Value};
use crate::{
    errors::ErrorKind,
    terms::{Term, Tuple as TupleT},
    types::Type,
};
use std::fmt;
use std::marker::PhantomData;
#[derive(Debug, PartialEq, Eq, Clone)]
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
    fn into_lambda<Ty1>(self) -> Result<Lambda<T, Ty1>, ErrorKind>
    where
        Ty1: Type,
    {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Lambda Abstraction".to_owned(),
        })
    }

    fn into_raise<Val, Ty1>(self) -> Result<Raise<Val, Ty1, T>, ErrorKind>
    where
        Val: Value<T>,
        Ty1: Type,
    {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Raise".to_owned(),
        })
    }
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
