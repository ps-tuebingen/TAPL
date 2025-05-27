use super::Value;
use std::{fmt, marker::PhantomData};
use syntax::terms::{Pair as PairT, Term};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pair<T, V>
where
    T: Term,
    V: Value,
{
    pub fst: Box<V>,
    pub snd: Box<V>,
    phantom: PhantomData<T>,
}

impl<T, V> Pair<T, V>
where
    T: Term,
    V: Value,
{
    pub fn new<V1, V2>(fst: V1, snd: V2) -> Pair<T, V>
    where
        V1: Into<V>,
        V2: Into<V>,
    {
        Pair {
            fst: Box::new(fst.into()),
            snd: Box::new(snd.into()),
            phantom: PhantomData,
        }
    }
}

impl<T, V> Value for Pair<T, V>
where
    T: Term,
    V: Value,
{
    type Term = PairT<T>;
}

impl<T, V> From<Pair<T, V>> for PairT<T>
where
    T: Term,
    V: Value,
{
    fn from(p: Pair<T, V>) -> PairT<T> {
        PairT::new(*p.fst, *p.snd)
    }
}

impl<T, V> fmt::Display for Pair<T, V>
where
    T: Term,
    V: Value,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {}, {} }}", self.fst, self.snd)
    }
}
