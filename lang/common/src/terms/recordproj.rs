use super::Term;
use crate::{
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    Label, TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordProj<T>
where
    T: LanguageTerm,
{
    record: Box<T>,
    label: Label,
}

impl<T> Term for RecordProj<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for RecordProj<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        RecordProj {
            record: Box::new(self.record.subst(v, t)),
            label: self.label,
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for RecordProj<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        RecordProj {
            record: Box::new(self.record.subst_type(v, ty)),
            label: self.label,
        }
        .into()
    }
}

impl<T> fmt::Display for RecordProj<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.record, self.label)
    }
}
