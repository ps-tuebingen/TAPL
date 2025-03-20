use super::Term;
use crate::Label;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub records: HashMap<Label, Term>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordProj {
    pub record: Box<Term>,
    pub label: Label,
}

impl From<Record> for Term {
    fn from(rec: Record) -> Term {
        Term::Record(rec)
    }
}

impl From<RecordProj> for Term {
    fn from(proj: RecordProj) -> Term {
        Term::RecordProj(proj)
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut recs: Vec<(&String, &Term)> = self.records.iter().collect();
        recs.sort_by(|(lb1, _), (lb2, _)| lb1.cmp(lb2));
        write!(
            f,
            "{{{}}}",
            recs.iter()
                .map(|(label, term)| format!("{label}={term}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl fmt::Display for RecordProj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.record, self.label)
    }
}
