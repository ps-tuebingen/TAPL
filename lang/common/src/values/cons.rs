use super::Value;
use crate::{language::LanguageTerm, terms::Cons as ConsT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cons<T>
where
    T: LanguageTerm,
{
    pub head: Box<<T as LanguageTerm>::Value>,
    pub tail: Box<<T as LanguageTerm>::Value>,
    ty: <T as LanguageTerm>::Type,
}

impl<T> Cons<T>
where
    T: LanguageTerm,
{
    pub fn new<T1, T2, Ty>(hd: T1, tl: T2, ty: Ty) -> Cons<T>
    where
        T1: Into<<T as LanguageTerm>::Value>,
        T2: Into<<T as LanguageTerm>::Value>,
        Ty: Into<<T as LanguageTerm>::Type>,
    {
        Cons {
            head: Box::new(hd.into()),
            tail: Box::new(tl.into()),
            ty: ty.into(),
        }
    }
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
        ConsT::new(*c.head, *c.tail, c.ty)
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
