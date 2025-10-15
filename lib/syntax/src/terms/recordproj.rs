use super::Term;
use crate::{
    Label, TypeVar, Var,
    language::Language,
    subst::{SubstTerm, SubstType},
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordProj<Lang>
where
    Lang: Language,
{
    pub record: Box<Lang::Term>,
    pub label: Label,
}

impl<Lang> RecordProj<Lang>
where
    Lang: Language,
{
    pub fn new<T1>(t: T1, lb: &str) -> RecordProj<Lang>
    where
        T1: Into<Lang::Term>,
    {
        RecordProj {
            record: Box::new(t.into()),
            label: lb.to_owned(),
        }
    }
}

impl<Lang> Term for RecordProj<Lang> where Lang: Language {}

impl<Lang> SubstTerm for RecordProj<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst(self, v: &Var, t: &<Lang as Language>::Term) -> Self::Target {
        RecordProj {
            record: Box::new(self.record.subst(v, t)),
            label: self.label,
        }
    }
}

impl<Lang> SubstType for RecordProj<Lang>
where
    Lang: Language,
{
    type Target = Self;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        RecordProj {
            record: Box::new(self.record.subst_type(v, ty)),
            label: self.label,
        }
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
