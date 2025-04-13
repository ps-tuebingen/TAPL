use super::Type;
use crate::Label;
use std::collections::HashMap;

pub struct Record<Ty>
where
    Ty: Type,
{
    records: HashMap<Label, Ty>,
}

impl<Ty> Type for Record<Ty> where Ty: Type {}
