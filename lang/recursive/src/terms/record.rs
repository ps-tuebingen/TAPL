use super::{Term, Var};
use crate::{
    traits::{
        is_value::IsValue,
        subst::{SubstTerm, SubstTy},
    },
    types::{Type, TypeVar},
    Label,
};
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    pub records: HashMap<Label, Term>,
}

impl Record {
    pub fn new(recs: &[(&str, Term)]) -> Record {
        let mut records = HashMap::new();
        for (lb, t) in recs.iter() {
            records.insert((*lb).to_owned(), t.clone());
        }
        Record { records }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordProj {
    pub record: Box<Term>,
    pub label: Label,
}

impl RecordProj {
    pub fn new(rec: Term, label: &str) -> RecordProj {
        RecordProj {
            record: Box::new(rec),
            label: label.to_owned(),
        }
    }
}

impl IsValue for Record {
    fn is_value(&self) -> bool {
        self.records.iter().all(|(_, t)| t.is_value())
    }
}
impl IsValue for RecordProj {
    fn is_value(&self) -> bool {
        false
    }
}

impl SubstTy for Record {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self {
        Record {
            records: self
                .records
                .into_iter()
                .map(|(label, term)| (label, term.subst_ty(v.clone(), ty.clone())))
                .collect(),
        }
    }
}

impl SubstTy for RecordProj {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self {
        RecordProj {
            record: Box::new(self.record.subst_ty(v.clone(), ty.clone())),
            label: self.label,
        }
    }
}

impl SubstTerm for Record {
    fn subst(self, v: Var, t: Term) -> Self {
        Record {
            records: self
                .records
                .into_iter()
                .map(|(label, term)| (label, term.subst(v.clone(), t.clone())))
                .collect(),
        }
    }
}

impl SubstTerm for RecordProj {
    fn subst(self, v: Var, t: Term) -> Self {
        RecordProj {
            record: Box::new(self.record.subst(v, t)),
            label: self.label,
        }
    }
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

impl fmt::Display for RecordProj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.record, self.label)
    }
}
