use super::Value;
use crate::{language::Language, terms::Tuple as TupleT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tuple<Lang>
where
    Lang: Language,
{
    pub vals: Vec<Lang::Value>,
}

impl<Lang> Tuple<Lang>
where
    Lang: Language,
{
    pub fn new<V1>(vals: Vec<V1>) -> Tuple<Lang>
    where
        V1: Into<Lang::Value>,
    {
        Tuple {
            vals: vals.into_iter().map(|v| v.into()).collect(),
        }
    }
}

impl<Lang> Value for Tuple<Lang>
where
    Lang: Language,
    TupleT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = TupleT<Lang>;
}

impl<Lang> From<Tuple<Lang>> for TupleT<Lang>
where
    Lang: Language,
{
    fn from(tup: Tuple<Lang>) -> TupleT<Lang> {
        TupleT::new(tup.vals)
    }
}

impl<Lang> fmt::Display for Tuple<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ts: Vec<String> = self.vals.iter().map(|t| t.to_string()).collect();
        ts.sort();
        write!(f, "( {} )", ts.join(", "))
    }
}
