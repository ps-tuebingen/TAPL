use super::Value;
use std::{collections::HashMap, fmt, marker::PhantomData};
use syntax::{
    terms::{Record as RecordT, Term},
    Label,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Record<T, V>
where
    T: Term,
    V: Value,
{
    pub records: HashMap<Label, V>,
    phantom: PhantomData<T>,
}

impl<T, V> Record<T, V>
where
    T: Term,
    V: Value,
{
    pub fn new<V1>(recs: HashMap<Label, V1>) -> Record<T, V>
    where
        V1: Into<V>,
    {
        Record {
            records: recs.into_iter().map(|(lb, t)| (lb, t.into())).collect(),
            phantom: PhantomData,
        }
    }
}

impl<T, V> Value for Record<T, V>
where
    T: Term,
    V: Value,
{
    type Term = RecordT<T>;
}

impl<T, V> From<Record<T, V>> for RecordT<T>
where
    T: Term,
    V: Value,
{
    fn from(rec: Record<T, V>) -> RecordT<T> {
        RecordT::new(rec.records)
    }
}

impl<T, V> fmt::Display for Record<T, V>
where
    T: Term,
    V: Value,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut recs: Vec<(&Label, &<T as Term>::Value)> = self.records.iter().collect();
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
