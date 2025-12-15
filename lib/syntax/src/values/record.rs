use super::Value;
use crate::{
    Label,
    language::Language,
    span::{Span, Spanned},
    terms::Record as RecordT,
};
use std::{collections::HashMap, fmt};

/// Record value
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Record<Lang>
where
    Lang: Language,
{
    /// Labeled values
    pub records: HashMap<Label, Lang::Value>,
    /// Source location
    pub span: Span,
}

impl<Lang> Record<Lang>
where
    Lang: Language,
{
    /// Create a new record value with given labeled values and source location
    #[must_use]
    pub fn new<V1>(recs: HashMap<Label, V1>, span: Span) -> Self
    where
        V1: Into<Lang::Value>,
    {
        Self {
            records: recs.into_iter().map(|(lb, t)| (lb, t.into())).collect(),
            span,
        }
    }
}

impl<Lang> Spanned for Record<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
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
    fn from(rec: Record<Lang>) -> Self {
        Self::new(rec.records, rec.span)
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
