use super::{Lambda, Raise, Value};
use crate::{
    errors::ErrorKind,
    terms::{Something as SomethingT, Term},
    types::Type,
};
use std::fmt;
use std::marker::PhantomData;
#[derive(Debug, PartialEq, Eq, Clone)]
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
