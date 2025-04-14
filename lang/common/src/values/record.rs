use super::Value;
use crate::Label;
use std::collections::HashMap;

pub struct Record<V>
where
    V: Value,
{
    records: HashMap<Label, V>,
}
