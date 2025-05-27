use super::Value;
use std::fmt;
use syntax::{
    terms::{Cons as ConsT, Term},
    types::Type,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cons<Ty, V>
where
    Ty: Type,
    V: Value,
{
    pub head: Box<V>,
    pub tail: Box<V>,
    ty: Ty,
}

impl<Ty, V> Cons<Ty, V>
where
    Ty: Type,
    V: Value,
{
    pub fn new<V1, V2, Typ>(hd: V1, tl: V2, ty: Typ) -> Cons<Ty, V>
    where
        V1: Into<V>,
        V2: Into<V>,
        Typ: Into<Ty>,
    {
        Cons {
            head: Box::new(hd.into()),
            tail: Box::new(tl.into()),
            ty: ty.into(),
        }
    }
}

impl<Ty, V> Value for Cons<Ty, V>
where
    Ty: Type,
    V: Value,
{
    type Term = ConsT<<V as Value>::Term, Ty>;
}

impl<Ty, V> From<Cons<Ty, V>> for ConsT<<V as Value>::Term, Ty>
where
    Ty: Type,
    V: Value,
{
    fn from(c: Cons<Ty, V>) -> ConsT<<V as Value>::Term, Ty> {
        ConsT::new(*c.head, *c.tail, c.ty)
    }
}

impl<Ty, V> fmt::Display for Cons<Ty, V>
where
    Ty: Type,
    V: Value,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cons[{}]({},{})", self.ty, self.head, self.tail)
    }
}
