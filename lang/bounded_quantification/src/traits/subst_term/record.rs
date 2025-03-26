use super::SubstTerm;
use crate::syntax::{Projection, Record, Term, Var};
use std::collections::HashMap;

impl SubstTerm for Record {
    fn subst(self, v: &Var, t: Term) -> Term {
        let mut new_recs = HashMap::new();
        for (lb, term) in self.records.into_iter() {
            let term_subst = term.subst(v, t.clone());
            new_recs.insert(lb, term_subst);
        }
        Record { records: new_recs }.into()
    }
}

impl SubstTerm for Projection {
    fn subst(self, v: &Var, t: Term) -> Term {
        let rec_subst = self.record.subst(v, t);
        Projection {
            record: Box::new(rec_subst),
            label: self.label,
        }
        .into()
    }
}
