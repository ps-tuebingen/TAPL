use super::Value;
use crate::{
    language::{LanguageTerm, LanguageValue},
    terms::Cons as ConsT,
};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cons<V>
where
    V: LanguageValue,
{
    head: V,
    tail: V,
    ty: <<V as LanguageValue>::Term as LanguageTerm>::Type,
}

impl<V> Value for Cons<V>
where
    V: LanguageValue,
{
    type Term = ConsT<<V as LanguageValue>::Term>;
}

impl<V> From<Cons<V>> for ConsT<<V as LanguageValue>::Term>
where
    V: LanguageValue,
{
    fn from(c: Cons<V>) -> ConsT<<V as LanguageValue>::Term> {
        ConsT::new(c.head, c.tail, c.ty)
    }
}

impl<V> fmt::Display for Cons<V>
where
    V: LanguageValue,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cons[{}]({},{})", self.ty, self.head, self.tail)
    }
}
