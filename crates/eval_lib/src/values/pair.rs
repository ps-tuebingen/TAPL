use super::Value;
use std::fmt;
use syntax::terms::Pair as PairT;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pair<V>
where
    V: Value,
{
    pub fst: Box<V>,
    pub snd: Box<V>,
}

impl<V> Pair<V>
where
    V: Value,
{
    pub fn new<V1, V2>(fst: V1, snd: V2) -> Pair<V>
    where
        V1: Into<V>,
        V2: Into<V>,
    {
        Pair {
            fst: Box::new(fst.into()),
            snd: Box::new(snd.into()),
        }
    }
}

impl<V> Value for Pair<V>
where
    V: Value,
{
    type Term = PairT<<V as Value>::Term>;
}

impl<V> From<Pair<V>> for PairT<<V as Value>::Term>
where
    V: Value,
{
    fn from(p: Pair<V>) -> PairT<<V as Value>::Term> {
        PairT::new(*p.fst, *p.snd)
    }
}

impl<V> fmt::Display for Pair<V>
where
    V: Value,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {}, {} }}", self.fst, self.snd)
    }
}
