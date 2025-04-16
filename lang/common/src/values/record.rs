use super::Value;
use crate::{language::LanguageTerm, terms::Record as RecordT, Label};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Record<T>
where
    T: LanguageTerm,
{
    records: HashMap<Label, <T as LanguageTerm>::Value>,
}

impl<T> Value for Record<T>
where
    T: LanguageTerm,
{
    type Term = RecordT<T>;
}

impl<T> From<Record<T>> for RecordT<T>
where
    T: LanguageTerm,
{
    fn from(rec: Record<T>) -> RecordT<T> {
        RecordT::new(rec.records)
    }
}

impl<T> fmt::Display for Record<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut recs: Vec<(&Label, &<T as LanguageTerm>::Value)> = self.records.iter().collect();
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
