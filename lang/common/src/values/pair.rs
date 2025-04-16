use super::Value;
use crate::{language::LanguageTerm, terms::Pair as PairT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pair<T>
where
    T: LanguageTerm,
{
    fst: Box<<T as LanguageTerm>::Value>,
    snd: Box<<T as LanguageTerm>::Value>,
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
