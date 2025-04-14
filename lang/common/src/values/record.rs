use super::Value;
use crate::Label;
use std::collections::HashMap;

pub struct Record {
    records: HashMap<Label, Value>,
}
