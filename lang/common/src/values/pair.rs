use super::Value;
use crate::terms::{Pair as PairT, Term};
use std::marker::PhantomData;

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
    T: Term,
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
