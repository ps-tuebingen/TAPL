use super::Term;
use crate::Label;

pub struct RecordProj<T>
where
    T: Term,
{
    record: T,
    label: Label,
}

impl<T> Term for RecordProj<T> where T: Term {}
