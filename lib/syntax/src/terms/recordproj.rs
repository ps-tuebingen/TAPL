use super::Term;
use crate::{
    Label, TypeVar, Var,
    language::Language,
    span::{Span, Spanned},
    subst::{SubstTerm, SubstType},
};
use macros::{EqNoSpan, HashNoSpan};
use std::{fmt, rc::Rc};

/// Term representing a record projection
#[derive(HashNoSpan, Clone, Debug, EqNoSpan)]
pub struct RecordProj<Lang>
where
    Lang: Language,
{
    /// term to project
    pub record: Rc<Lang::Term>,
    /// Label to project onto
    pub label: Label,
    /// Source location
    pub span: Span,
}

impl<Lang> RecordProj<Lang>
where
    Lang: Language,
{
    /// Create a new record projection from given term, label and span
    pub fn new<T1>(t: T1, lb: &str, span: Span) -> Self
    where
        T1: Into<Lang::Term>,
    {
        Self {
            record: Rc::new(t.into()),
            label: lb.to_owned(),
            span,
        }
    }
}

impl<Lang> Spanned for RecordProj<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> Term for RecordProj<Lang> where Lang: Language {}

impl<Lang> SubstTerm for RecordProj<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(mut self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        self.record = self.record.subst(v, t);
        self
    }
}

impl<Lang> SubstType for RecordProj<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(mut self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        self.record = self.record.subst_type(v, ty);
        self
    }
}

impl<Lang> fmt::Display for RecordProj<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}).{}", self.record, self.label)
    }
}
