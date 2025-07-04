use super::Term;
use crate::{
    Label, TypeVar, Var,
    subst::{SubstTerm, SubstType},
    types::Type,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordProj<T>
where
    T: Term,
{
    pub record: Box<T>,
    pub label: Label,
}

impl<T> RecordProj<T>
where
    T: Term,
{
    pub fn new<T1>(t: T1, lb: &str) -> RecordProj<T>
    where
        T1: Into<T>,
    {
        RecordProj {
            record: Box::new(t.into()),
            label: lb.to_owned(),
        }
    }
}

impl<T> Term for RecordProj<T> where T: Term {}

impl<T> SubstTerm<T> for RecordProj<T>
where
    T: Term + SubstTerm<T, Target = T>,
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

impl<T, Ty> SubstType<Ty> for RecordProj<T>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        RecordProj {
            record: Box::new(self.record.subst_type(v, ty)),
            label: self.label,
        }
        .into()
    }
}

impl<T> fmt::Display for RecordProj<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}).{}", self.record, self.label)
    }
}
