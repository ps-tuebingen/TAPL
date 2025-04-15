use super::Value;
use crate::terms::{Pair as PairT, Term};
use std::fmt;
use std::marker::PhantomData;
#[derive(Debug)]
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
