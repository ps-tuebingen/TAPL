use super::Type;
use crate::Label;
use std::collections::HashMap;

pub struct Variant<Ty>
where
    Ty: Type,
{
    variants: HashMap<Label, Ty>,
}

impl<Ty> Type for Variant<Ty> where Ty: Type {}
