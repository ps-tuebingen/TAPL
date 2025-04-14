use super::Term;
use crate::{
    subst::{SubstTerm, SubstType},
    types::Type,
    Label, TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug)]
pub struct RecordProj<T>
where
    T: Term,
{
    record: Box<T>,
    label: Label,
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
        write!(f, "{}.{}", self.record, self.label)
    }
}
