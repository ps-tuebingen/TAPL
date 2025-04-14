use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    Label, TypeVar, Var,
};
use std::{collections::HashMap, fmt};

#[derive(Clone, Debug)]
pub struct Record<T>
where
    T: Term,
{
    records: HashMap<Label, T>,
}

impl<T> Term for Record<T> where T: Term {}

impl<T> SubstTerm<T> for Record<T>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Record {
            records: self
                .records
                .into_iter()
                .map(|(lb, t1)| (lb, t1.subst(v, t)))
                .collect(),
        }
        .into()
    }
}

impl<T, Ty> SubstType<Ty> for Record<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Record {
            records: self
                .records
                .into_iter()
                .map(|(lb, t)| (lb, t.subst_type(v, ty)))
                .collect(),
        }
        .into()
    }
}

impl<T> fmt::Display for Record<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut recs: Vec<(&Label, &T)> = self.records.iter().collect();
        recs.sort_by(|(lb1, _), (lb2, _)| lb1.cmp(lb2));
        write!(
            f,
            "{{ {} }}",
            recs.iter()
                .map(|(lb, t)| format!("{lb} = {t}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
