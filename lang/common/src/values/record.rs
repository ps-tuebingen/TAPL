use super::Value;
use crate::{
    terms::{Record as RecordT, Term},
    Label,
};
use std::{collections::HashMap, marker::PhantomData};

pub struct Record<V, T>
where
    V: Value<T>,
    T: Term,
{
    records: HashMap<Label, V>,
    phantom: PhantomData<T>,
}

impl<V, T> Value<T> for Record<V, T>
where
    V: Value<T> + Into<T>,
    T: Term,
{
    type Term = RecordT<T>;
}

impl<V, T> From<Record<V, T>> for RecordT<T>
where
    V: Value<T> + Into<T>,
    T: Term,
{
    fn from(rec: Record<V, T>) -> RecordT<T> {
        RecordT::new(rec.records)
    }
}
