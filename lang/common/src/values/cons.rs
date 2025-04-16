use super::Value;
use crate::{language::LanguageTerm, terms::Cons as ConsT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cons<T>
where
    T: LanguageTerm,
{
    head: <T as LanguageTerm>::Value,
    tail: <T as LanguageTerm>::Value,
    ty: <T as LanguageTerm>::Type,
}

impl<T> Value for Cons<T>
where
    T: LanguageTerm,
{
    type Term = ConsT<T>;
}

impl<T> From<Cons<T>> for ConsT<T>
where
    T: LanguageTerm,
{
    fn from(c: Cons<T>) -> ConsT<T> {
        ConsT::new(c.head, c.tail, c.ty)
    }
}

impl<T> fmt::Display for Cons<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cons[{}]({},{})", self.ty, self.head, self.tail)
    }
}
