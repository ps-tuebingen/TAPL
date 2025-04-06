use super::Term;
use crate::Label;
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub struct Record {
    pub records: HashMap<Label, Term>,
}

impl Record {
    pub fn new(recs: Vec<(&str, Term)>) -> Record {
        Record {
            records: recs
                .into_iter()
                .map(|(label, term)| (label.to_owned(), term))
                .collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RecordProj {
    pub term: Box<Term>,
    pub label: Label,
}

impl RecordProj {
    pub fn new<T: Into<Term>>(term: T, label: &str) -> RecordProj {
        RecordProj {
            term: Box::new(term.into()),
            label: label.to_owned(),
        }
    }
}

impl From<Record> for Term {
    fn from(record: Record) -> Term {
        Term::Record(record)
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
            "{{ {} }}",
            recs.iter()
                .map(|(label, term)| format!("{label} = {term}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl fmt::Display for RecordProj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}.{})", self.term, self.label)
    }
}
