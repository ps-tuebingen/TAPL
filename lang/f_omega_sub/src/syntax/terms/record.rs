use super::{Term, Var};
use crate::{
    syntax::types::{Type, TypeVar},
    traits::{SubstTerm, SubstTy},
    Label,
};
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub struct Record {
    pub records: HashMap<Label, Term>,
}

impl Record {
    pub fn new(recs: Vec<(&str, Term)>) -> Record {
        let mut records = HashMap::new();
        for (label, term) in recs {
            records.insert(label.to_owned(), term);
        }
        Record { records }
    }
}

impl SubstTerm for Record {
    fn subst(self, v: &Var, t: Term) -> Term {
        Record {
            records: self
                .records
                .into_iter()
                .map(|(label, term)| (label, term.subst(v, t.clone())))
                .collect(),
        }
        .into()
    }
}

impl SubstTy for Record {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        Record {
            records: self
                .records
                .into_iter()
                .map(|(label, term)| (label, term.subst_ty(v, ty.clone())))
                .collect(),
        }
    }
}

impl From<Record> for Term {
    fn from(rec: Record) -> Term {
        Term::Record(rec)
    }
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut recs: Vec<(&Label, &Term)> = self.records.iter().collect();
        recs.sort_by(|(lb1, _), (lb2, _)| lb1.cmp(lb2));
        write!(
            f,
            "{{ {} }}",
            recs.iter()
                .map(|(label, term)| format!("{label}={term}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
