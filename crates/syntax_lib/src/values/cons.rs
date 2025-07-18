use crate::{terms::Cons as ConsT, types::Type, values::Value};
use errors::ValueKind;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cons<V, Ty>
where
    Ty: Type,
    V: Value,
{
    pub head: Box<V>,
    pub tail: Box<V>,
    pub ty: Ty,
}

impl<V, Ty> Cons<V, Ty>
where
    Ty: Type,
    V: Value,
{
    pub fn new<V1, V2, Typ>(hd: V1, tl: V2, ty: Typ) -> Cons<V, Ty>
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

impl<V, Ty> Value for Cons<V, Ty>
where
    Ty: Type,
    V: Value,
{
    type Term = ConsT<<V as Value>::Term, Ty>;
    fn knd(&self) -> ValueKind {
        ValueKind::Cons
    }
}

impl<V, Ty> From<Cons<V, Ty>> for ConsT<<V as Value>::Term, Ty>
where
    Ty: Type,
    V: Value,
{
    fn from(c: Cons<V, Ty>) -> ConsT<<V as Value>::Term, Ty> {
        ConsT::new(*c.head, *c.tail, c.ty)
    }
}

impl<V, Ty> fmt::Display for Cons<V, Ty>
where
    Ty: Type,
    V: Value,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cons[{}]({},{})", self.ty, self.head, self.tail)
    }
}
