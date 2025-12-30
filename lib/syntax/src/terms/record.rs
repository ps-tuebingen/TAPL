use super::Term;
use crate::{
    Label, TypeVar, Var,
    free_vars::{FreeTypeVars, FreeVars},
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::EqNoSpan;
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

/// Term representing a record
#[derive(Clone, Debug, EqNoSpan)]
pub struct Record<Lang>
where
    Lang: Language,
{
    /// Terms with their corresponding labels
    pub records: HashMap<Label, Lang::Term>,
    /// Source location
    pub span: Span,
}

impl<Lang> Record<Lang>
where
    Lang: Language,
{
    #[must_use]
    /// Create a new record with given records and span
    pub fn new<T1>(recs: HashMap<Label, T1>, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
    {
        Self {
            records: recs.into_iter().map(|(lb, t)| (lb, t.into())).collect(),
            span,
        }
    }
}

impl<Lang> Spanned for Record<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> FreeVars for Record<Lang>
where
    Lang: Language,
{
    fn free_vars(&self, vars: &mut HashSet<Var>) {
        for t in self.records.values() {
            t.free_vars(vars);
        }
    }
}

impl<Lang> FreeTypeVars for Record<Lang>
where
    Lang: Language,
{
    fn free_type_vars(&self, vars: &mut HashSet<TypeVar>) {
        for t in self.records.values() {
            t.free_type_vars(vars);
        }
    }
}

impl<Lang> Term for Record<Lang> where Lang: Language {}

impl<Lang> SubstTerm for Record<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(mut self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        self.records = self
            .records
            .into_iter()
            .map(|(lb, t1)| (lb, t1.subst(v, t)))
            .collect();
        self
    }
}

impl<Lang> SubstType for Record<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.records = self
            .records
            .into_iter()
            .map(|(lb, t)| (lb, t.subst_type(v, ty)))
            .collect();
        self
    }
}

impl<Lang> fmt::Display for Record<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut recs: Vec<(&Label, &<Lang as Language>::Term)> = self.records.iter().collect();
        recs.sort_by(|(lb1, _), (lb2, _)| lb1.cmp(lb2));
        write!(
            f,
            "{{{}}}",
            recs.iter()
                .map(|(lb, t)| format!("{lb}={t}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
