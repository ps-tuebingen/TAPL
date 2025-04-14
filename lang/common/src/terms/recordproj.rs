use super::Term;
use crate::Label;
use std::fmt;

#[derive(Clone, Debug)]
pub struct RecordProj<T>
where
    T: Term,
{
    record: T,
    label: Label,
}

impl<T> Term for RecordProj<T> where T: Term {}

impl<T> fmt::Display for RecordProj<T>
where
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.record, self.label)
    }
}
