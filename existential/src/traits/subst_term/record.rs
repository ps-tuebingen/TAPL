use super::SubstTerm;
use crate::terms::{Record, RecordProj, Term, Var};

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
impl SubstTerm for RecordProj {
    fn subst(self, v: &Var, t: Term) -> Term {
        RecordProj {
            term: Box::new(self.term.subst(v, t)),
            label: self.label,
        }
        .into()
    }
}
