use super::Value;
use crate::errors::ValueKind;
use std::fmt;
use syntax::terms::Something as SomethingT;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Something<V>
where
    V: Value,
{
    pub val: Box<V>,
}

impl<V> Something<V>
where
    V: Value,
{
    pub fn new<V1>(v: V1) -> Something<V>
    where
        V1: Into<V>,
    {
        Something {
            val: Box::new(v.into()),
        }
    }
}

impl<V> Value for Something<V>
where
    V: Value,
{
    type Term = SomethingT<<V as Value>::Term>;

    fn knd(&self) -> ValueKind {
        ValueKind::Something
    }
}

impl<V> From<Something<V>> for SomethingT<<V as Value>::Term>
where
    V: Value,
{
    fn from(something: Something<V>) -> SomethingT<<V as Value>::Term> {
        SomethingT::new(*something.val)
    }
}

impl<V> fmt::Display for Something<V>
where
    V: Value,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something({})", self.val)
    }
}
