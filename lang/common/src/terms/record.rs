use super::Term;
use crate::Label;
use std::collections::HashMap;
use std::fmt;
#[derive(Clone, Debug)]

pub struct Record<T>
where
    T: Term,
{
    records: HashMap<Label, T>,
}

impl<T> Term for Record<T> where T: Term {}

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
