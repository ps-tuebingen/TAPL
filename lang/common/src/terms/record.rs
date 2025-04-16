use super::Term;
use crate::{
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    Label, TypeVar, Var,
};
use std::{collections::HashMap, fmt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record<T>
where
    T: LanguageTerm,
{
    records: HashMap<Label, T>,
}

impl<T> Record<T>
where
    T: LanguageTerm,
{
    pub fn new<T1>(recs: HashMap<Label, T1>) -> Record<T>
    where
        T1: Into<T>,
    {
        Record {
            records: recs.into_iter().map(|(lb, t)| (lb, t.into())).collect(),
        }
    }
}

impl<T> Term for Record<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Record<T>
where
    T: LanguageTerm,
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

impl<T> SubstType<<T as LanguageTerm>::Type> for Record<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
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
    T: LanguageTerm,
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
