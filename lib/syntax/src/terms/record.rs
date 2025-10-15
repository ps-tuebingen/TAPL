use super::Term;
use crate::{
    Label, TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::{collections::HashMap, fmt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record<Lang>
where
    Lang: Language,
{
    pub records: HashMap<Label, Lang::Term>,
}

impl<Lang> Record<Lang>
where
    Lang: Language,
{
    pub fn new<T1>(recs: HashMap<Label, T1>) -> Record<Lang>
    where
        T1: Into<Lang::Term>,
    {
        Record {
            records: recs.into_iter().map(|(lb, t)| (lb, t.into())).collect(),
        }
    }
}

impl<Lang> Term for Record<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Record<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        Record {
            records: self
                .records
                .into_iter()
                .map(|(lb, t1)| (lb, t1.subst(v, t)))
                .collect(),
        }
    }
}

impl<Lang> SubstType for Record<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Record {
            records: self
                .records
                .into_iter()
                .map(|(lb, t)| (lb, t.subst_type(v, ty)))
                .collect(),
        }
    }
}

impl<Lang> fmt::Display for Record<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut recs: Vec<(&Label, &<Lang as Language>::Term)> = self.records.iter().collect();
        recs.sort_by(|(lb1, _), (lb2, _)| lb1.cmp(lb2));
        write!(
            f,
            "{{{}}}",
            recs.iter()
                .map(|(lb, t)| format!("{lb}={t}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
