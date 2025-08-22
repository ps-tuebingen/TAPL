use super::Value;
use crate::{language::Language, terms::Pair as PairT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pair<Lang>
where
    Lang: Language,
{
    pub fst: Box<Lang::Value>,
    pub snd: Box<Lang::Value>,
}

impl<Lang> Pair<Lang>
where
    Lang: Language,
{
    pub fn new<V1, V2>(fst: V1, snd: V2) -> Pair<Lang>
    where
        V1: Into<Lang::Value>,
        V2: Into<Lang::Value>,
    {
        Pair {
            fst: Box::new(fst.into()),
            snd: Box::new(snd.into()),
        }
    }
}

impl<Lang> Value for Pair<Lang>
where
    Lang: Language,
    PairT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = PairT<Lang>;
}

impl<Lang> From<Pair<Lang>> for PairT<Lang>
where
    Lang: Language,
{
    fn from(p: Pair<Lang>) -> PairT<Lang> {
        PairT::new(*p.fst, *p.snd)
    }
}

impl<Lang> fmt::Display for Pair<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ {}, {} }}", self.fst, self.snd)
    }
}
