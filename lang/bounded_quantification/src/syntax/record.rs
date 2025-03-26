use super::{Label, Term};
use std::{collections::HashMap, fmt};

#[derive(Clone, Debug)]
pub struct Record {
    pub records: HashMap<Label, Term>,
}

#[derive(Clone, Debug)]
pub struct Projection {
    pub record: Box<Term>,
    pub label: Label,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut recs: Vec<(&String, &Term)> = self.records.iter().collect();
        recs.sort_by(|(lb1, _), (lb2, _)| lb1.cmp(&lb2));
        let rec_strs: Vec<String> = recs.iter().map(|(lb, t)| format!("{lb} = {t}")).collect();
        write!(f, "{{ {} }}", rec_strs.join(", "))
    }
}

impl fmt::Display for Projection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.record, self.label)
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
