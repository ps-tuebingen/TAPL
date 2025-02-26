use super::SubstTy;
use crate::{
    terms::{Record, RecordProj},
    types::{Type, TypeVar},
};

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
impl SubstTy for RecordProj {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        RecordProj {
            term: Box::new(self.term.subst_ty(v, ty)),
            label: self.label,
        }
    }
}
