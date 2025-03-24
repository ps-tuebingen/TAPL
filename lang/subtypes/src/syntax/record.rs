use super::{Label, Term};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct Record {
    pub records: HashMap<Label, Term>,
}

impl Record {
    pub fn new(recs: Vec<(&str, Term)>) -> Record {
        let recs: Vec<(String, Term)> =
            recs.into_iter().map(|(lb, t)| (lb.to_owned(), t)).collect();
        Record {
            records: HashMap::from_iter(recs),
        }
    }
}

#[derive(Debug)]
pub struct Projection {
    pub record: Box<Term>,
    pub label: Label,
}

impl Projection {
    pub fn new(t: Term, l: &str) -> Projection {
        Projection {
            record: Box::new(t),
            label: l.to_owned(),
        }
    }
}

impl From<Record> for Term {
    fn from(rec: Record) -> Term {
        Term::Record(rec)
    }
}

impl From<Projection> for Term {
    fn from(proj: Projection) -> Term {
        Term::Projection(proj)
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let recs: Vec<String> = self
            .records
            .iter()
            .map(|(label, term)| format!("{label}={term}"))
            .collect();
        write!(f, "{{ {} }}", recs.join(", "))
    }
}

impl fmt::Display for Projection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}).{}", self.record, self.label)
    }
}
