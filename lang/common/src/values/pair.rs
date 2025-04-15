use super::{Lambda, Raise, Value};
use crate::{
    errors::ErrorKind,
    terms::{Pair as PairT, Term},
    types::Type,
};
use std::fmt;
use std::marker::PhantomData;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pair<V, T>
where
    V: Value<T>,
    T: Term,
{
    fst: Box<V>,
    snd: Box<V>,
    phantom: PhantomData<T>,
}

impl<V, T> Value<T> for Pair<V, T>
where
    V: Value<T> + Into<T>,
    T: Term + From<PairT<T>>,
{
    type Term = PairT<T>;
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

impl<V, T> From<Pair<V, T>> for PairT<T>
where
    T: Term,
    V: Value<T> + Into<T>,
{
    fn from(p: Pair<V, T>) -> PairT<T> {
        PairT::new(*p.fst, *p.snd)
    }
}

impl<V, T> fmt::Display for Pair<V, T>
where
    V: Value<T>,
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {}, {} }}", self.fst, self.snd)
    }
}
