use super::SubstTy;
use crate::{
    syntax::{Projection, Record},
    types::{Type, TypeVar},
};
use std::collections::HashMap;

impl SubstTy for Record {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        let mut new_recs = HashMap::new();
        for (lb, term) in self.records.into_iter() {
            let term_subst = term.subst_ty(v, ty.clone());
            new_recs.insert(lb, term_subst);
        }
        Record { records: new_recs }
    }
}

impl SubstTy for Projection {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        let rec_subst = self.record.subst_ty(v, ty);
        Projection {
            record: Box::new(rec_subst),
            label: self.label,
        }
    }
}
