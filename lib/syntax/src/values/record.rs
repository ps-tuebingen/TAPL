use super::Value;
use crate::{Label, language::Language, terms::Record as RecordT};
use std::{collections::HashMap, fmt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Record<Lang>
where
    Lang: Language,
{
    pub records: HashMap<Label, Lang::Value>,
}

impl<Lang> Record<Lang>
where
    Lang: Language,
{
    pub fn new<V1>(recs: HashMap<Label, V1>) -> Record<Lang>
    where
        V1: Into<Lang::Value>,
    {
        Record {
            records: recs.into_iter().map(|(lb, t)| (lb, t.into())).collect(),
        }
    }
}

impl<Lang> Value for Record<Lang>
where
    Lang: Language,
    RecordT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = RecordT<Lang>;
}

impl<Lang> From<Record<Lang>> for RecordT<Lang>
where
    Lang: Language,
{
    fn from(rec: Record<Lang>) -> RecordT<Lang> {
        RecordT::new(rec.records)
    }
}

impl<Lang> fmt::Display for Record<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut recs: Vec<(&Label, &Lang::Value)> = self.records.iter().collect();
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
