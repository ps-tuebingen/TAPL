use super::Subst;
use crate::{
    terms::syntax::{Record, RecordProj, Term},
    Var,
};

impl Subst for Record {
    type Target = Record;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        Record {
            records: self
                .records
                .into_iter()
                .map(|(label, t)| (label, t.subst(var.clone(), term.clone())))
                .collect(),
        }
    }
}

impl Subst for RecordProj {
    type Target = RecordProj;
    fn subst(self, var: Var, term: Term) -> Self::Target {
        RecordProj {
            record: self.record.subst(var, term),
            label: self.label,
        }
    }
}
