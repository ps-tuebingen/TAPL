use super::Value;
use crate::{language::LanguageTerm, terms::Something as SomethingT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Something<T>
where
    T: LanguageTerm,
{
    val: Box<<T as LanguageTerm>::Value>,
}

impl<T> Value for Something<T>
where
    T: LanguageTerm,
{
    type Term = SomethingT<T>;
}

impl<T> From<Something<T>> for SomethingT<T>
where
    T: LanguageTerm,
{
    fn from(something: Something<T>) -> SomethingT<T> {
        SomethingT::new(*something.val)
    }
}

impl<T> fmt::Display for Something<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something({})", self.val)
    }
}
