use super::Value;
use crate::{
    terms::{Record as RecordT, Term},
    Label,
};
use std::fmt;
use std::{collections::HashMap, marker::PhantomData};

#[derive(Debug, Clone)]
pub struct Record<V, T>
where
    V: Value<T>,
    T: Term,
{
    records: HashMap<Label, V>,
    phantom: PhantomData<T>,
}

impl<V, T> Value<T> for Record<V, T>
where
    V: Value<T> + Into<T>,
    T: Term + From<RecordT<T>>,
{
    type Term = RecordT<T>;
}

impl<V, T> From<Record<V, T>> for RecordT<T>
where
    V: Value<T> + Into<T>,
    T: Term,
{
    fn from(rec: Record<V, T>) -> RecordT<T> {
        RecordT::new(rec.records)
    }
}

impl<V, T> fmt::Display for Record<V, T>
where
    V: Value<T>,
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut recs: Vec<(&Label, &V)> = self.records.iter().collect();
        recs.sort_by(|(lb1, _), (lb2, _)| lb1.cmp(lb2));
        write!(
            f,
            "{{ {} }}",
            recs.iter()
                .map(|(lb, t)| format!("{lb} = {t}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
