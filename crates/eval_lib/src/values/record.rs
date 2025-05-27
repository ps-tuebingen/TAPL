use super::Value;
use std::{collections::HashMap, fmt};
use syntax::{terms::Record as RecordT, Label};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Record<V>
where
    V: Value,
{
    pub records: HashMap<Label, V>,
}

impl<V> Record<V>
where
    V: Value,
{
    pub fn new<V1>(recs: HashMap<Label, V1>) -> Record<V>
    where
        V1: Into<V>,
    {
        Record {
            records: recs.into_iter().map(|(lb, t)| (lb, t.into())).collect(),
        }
    }
}

impl<V> Value for Record<V>
where
    V: Value,
{
    type Term = RecordT<<V as Value>::Term>;
}

impl<V> From<Record<V>> for RecordT<<V as Value>::Term>
where
    V: Value,
{
    fn from(rec: Record<V>) -> RecordT<<V as Value>::Term> {
        RecordT::new(rec.records)
    }
}

impl<V> fmt::Display for Record<V>
where
    V: Value,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut recs: Vec<(&Label, &V)> = self.records.iter().collect();
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
