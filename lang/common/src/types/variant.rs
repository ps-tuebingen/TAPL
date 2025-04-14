use super::Type;
use crate::Label;
use std::collections::HashMap;
use std::fmt;
#[derive(Clone, Debug)]

pub struct Variant<Ty>
where
    Ty: Type,
{
    variants: HashMap<Label, Ty>,
}

impl<Ty> Type for Variant<Ty> where Ty: Type {}

impl<Ty> fmt::Display for Variant<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut vars: Vec<(&Label, &Ty)> = self.variants.iter().collect();
        vars.sort_by(|(lb1, _), (lb2, _)| lb1.cmp(lb2));
        write!(
            f,
            "< {} >",
            vars.iter()
                .map(|(lb, ty)| format!("{lb} : {ty}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
