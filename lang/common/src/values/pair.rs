use super::Value;
use crate::{language::LanguageTerm, terms::Pair as PairT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pair<T>
where
    T: LanguageTerm,
{
    pub fst: Box<<T as LanguageTerm>::Value>,
    pub snd: Box<<T as LanguageTerm>::Value>,
}

impl<T> Pair<T>
where
    T: LanguageTerm,
{
    pub fn new<V1, V2>(fst: V1, snd: V2) -> Pair<T>
    where
        V1: Into<<T as LanguageTerm>::Value>,
        V2: Into<<T as LanguageTerm>::Value>,
    {
        Pair {
            fst: Box::new(fst.into()),
            snd: Box::new(snd.into()),
        }
    }
}

impl<T> Value for Pair<T>
where
    T: LanguageTerm,
{
    type Term = PairT<T>;
}

impl<T> From<Pair<T>> for PairT<T>
where
    T: LanguageTerm,
{
    fn from(p: Pair<T>) -> PairT<T> {
        PairT::new(*p.fst, *p.snd)
    }
}

impl<T> fmt::Display for Pair<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {}, {} }}", self.fst, self.snd)
    }
}
