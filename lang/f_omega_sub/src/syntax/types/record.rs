use super::{Type, TypeVar};
use crate::{traits::SubstTy, Label};
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordTy {
    pub records: HashMap<Label, Type>,
}

impl RecordTy {
    pub fn new(recs: Vec<(&str, Type)>) -> RecordTy {
        let mut records = HashMap::new();
        for (label, ty) in recs {
            records.insert(label.to_owned(), ty);
        }
        RecordTy { records }
    }
}

impl SubstTy for RecordTy {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        RecordTy {
            records: self
                .records
                .into_iter()
                .map(|(label, typ)| (label, typ.subst_ty(v, ty.clone())))
                .collect(),
        }
    }
}

impl From<RecordTy> for Type {
    fn from(rec: RecordTy) -> Type {
        Type::Record(rec)
    }
}

impl fmt::Display for RecordTy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{ {} }}",
            self.records
                .iter()
                .map(|(label, ty)| format!("{label}:{ty}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
