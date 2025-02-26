use super::{Term, Var};
use crate::{
    syntax::{
        types::{Type, TypeVar},
        Label,
    },
    traits::{SubstTerm, SubstTy},
};
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub struct Record {
    pub records: HashMap<Label, Term>,
}

impl Record {
    pub fn new(recs: Vec<(&str, Term)>) -> Record {
        let mut records = HashMap::new();
        for (lb, t) in recs {
            records.insert(lb.to_owned(), t);
        }
        Record { records }
    }
}

impl From<Record> for Term {
    fn from(rec: Record) -> Term {
        Term::Record(rec)
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

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ {} }}",
            self.records
                .iter()
                .map(|(label, term)| format!("{label} = {term}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
