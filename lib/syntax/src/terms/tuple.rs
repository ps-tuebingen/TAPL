use super::Term;
use crate::{
    TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tuple<Lang>
where
    Lang: Language,
{
    pub terms: Vec<Lang::Term>,
}

impl<Lang> Tuple<Lang>
where
    Lang: Language,
{
    pub fn new<T1>(ts: Vec<T1>) -> Tuple<Lang>
    where
        T1: Into<Lang::Term>,
    {
        Tuple {
            terms: ts.into_iter().map(|t| t.into()).collect(),
        }
    }
}

impl<Lang> Term for Tuple<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Tuple<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Tuple {
            terms: self.terms.into_iter().map(|t1| t1.subst(v, t)).collect(),
        }
    }
}

impl<Lang> SubstType for Tuple<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Tuple {
            terms: self
                .terms
                .into_iter()
                .map(|t| t.subst_type(v, ty))
                .collect(),
        }
    }
}

impl<Lang> fmt::Display for Tuple<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ts: Vec<String> = self.terms.iter().map(|t| t.to_string()).collect();
        ts.sort();
        write!(f, "( {} )", ts.join(", "))
    }
}
