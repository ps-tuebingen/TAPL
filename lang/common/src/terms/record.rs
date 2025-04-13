use super::Term;
use crate::Label;
use std::collections::HashMap;

pub struct Record<T>
where
    T: Term,
{
    records: HashMap<Label, T>,
}

impl<T> Term for Record<T> where T: Term {}
