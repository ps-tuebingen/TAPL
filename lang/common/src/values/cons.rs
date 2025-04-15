use super::{Lambda, Raise, Value};
use crate::{
    errors::ErrorKind,
    terms::{Cons as ConsT, Term},
    types::Type,
};
use std::fmt;
use std::marker::PhantomData;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Cons<V, Ty, T>
where
    V: Value<T>,
    Ty: Type,
    T: Term,
{
    head: V,
    tail: V,
    ty: Ty,
    phantom: PhantomData<T>,
}

impl<V, Ty, T> Value<T> for Cons<V, Ty, T>
where
    V: Value<T> + Into<T>,
    T: Term + From<ConsT<T, Ty>>,
    Ty: Type,
{
    type Term = ConsT<T, Ty>;
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

impl<V, Ty, T> From<Cons<V, Ty, T>> for ConsT<T, Ty>
where
    T: Term,
    V: Value<T> + Into<T>,
    Ty: Type,
{
    fn from(c: Cons<V, Ty, T>) -> ConsT<T, Ty> {
        ConsT::new(c.head, c.tail, c.ty)
    }
}

impl<V, Ty, T> fmt::Display for Cons<V, Ty, T>
where
    V: Value<T>,
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "cons[{}]({},{})", self.ty, self.head, self.tail)
    }
}
