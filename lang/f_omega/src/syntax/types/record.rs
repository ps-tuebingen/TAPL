use super::{Type, TypeVar};
use crate::{errors::ErrorKind, syntax::Label, traits::SubstTy};
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecTy {
    pub records: HashMap<Label, Type>,
}

impl RecTy {
    pub fn new<T: Into<Type>>(recs: Vec<(&str, T)>) -> RecTy {
        let mut records = HashMap::new();
        for (lb, ty) in recs {
            records.insert(lb.to_owned(), ty.into());
        }
        RecTy { records }
    }

    pub fn lookup(&self, label: &Label) -> Result<Type, ErrorKind> {
        self.records
            .get(label)
            .cloned()
            .ok_or(ErrorKind::UndefinedLabel(label.clone()))
    }
}

impl From<RecTy> for Type {
    fn from(rec: RecTy) -> Type {
        Type::Record(rec)
    }
}

impl SubstTy for RecTy {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        RecTy {
            records: self
                .records
                .into_iter()
                .map(|(label, typ)| (label, typ.subst_ty(v, ty.clone())))
                .collect(),
        }
    }
}

impl fmt::Display for RecTy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut recs: Vec<(&String, &Type)> = self.records.iter().collect();
        recs.sort_by(|(lb1, _), (lb2, _)| lb1.cmp(lb2));
        write!(
            f,
            "{{ {} }}",
            recs.iter()
                .map(|(label, ty)| format!("{label} : {ty}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
