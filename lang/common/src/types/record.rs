use super::{Fun, Type};
use crate::{errors::ErrorKind, subst::SubstType, Label, TypeVar};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record<Ty>
where
    Ty: Type,
{
    records: HashMap<Label, Ty>,
}

impl<Ty> Type for Record<Ty>
where
    Ty: Type,
{
    fn into_fun<Ty1>(self) -> Result<Fun<Ty1>, ErrorKind>
    where
        Ty1: Type,
    {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Function Type".to_owned(),
        })
    }
}

impl<Ty> SubstType<Ty> for Record<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Record {
            records: self
                .records
                .into_iter()
                .map(|(lb, ty1)| (lb, ty1.subst_type(v, ty)))
                .collect(),
        }
        .into()
    }
}

impl<Ty> fmt::Display for Record<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut recs: Vec<(&String, &Ty)> = self.records.iter().collect();
        recs.sort_by(|(lb1, _), (lb2, _)| lb1.cmp(lb2));
        write!(
            f,
            "{{ {} }}",
            recs.iter()
                .map(|(lb, ty)| format!("{lb} : {ty}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
