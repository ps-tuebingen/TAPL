use crate::{language::Language, terms::Cons as ConsT, values::Value};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cons<Lang>
where
    Lang: Language,
{
    pub head: Box<Lang::Value>,
    pub tail: Box<Lang::Value>,
    pub ty: Lang::Type,
}

impl<Lang> Cons<Lang>
where
    Lang: Language,
{
    pub fn new<V1, V2, Typ>(hd: V1, tl: V2, ty: Typ) -> Cons<Lang>
    where
        V1: Into<Lang::Value>,
        V2: Into<Lang::Value>,
        Typ: Into<Lang::Type>,
    {
        Cons {
            head: Box::new(hd.into()),
            tail: Box::new(tl.into()),
            ty: ty.into(),
        }
    }
}

impl<Lang> Value for Cons<Lang>
where
    Lang: Language,
    ConsT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = ConsT<Lang>;
}

impl<Lang> From<Cons<Lang>> for ConsT<Lang>
where
    Lang: Language,
{
    fn from(c: Cons<Lang>) -> ConsT<Lang> {
        ConsT::new(*c.head, *c.tail, c.ty)
    }
}

impl<Lang> fmt::Display for Cons<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cons[{}]({},{})", self.ty, self.head, self.tail)
    }
}
