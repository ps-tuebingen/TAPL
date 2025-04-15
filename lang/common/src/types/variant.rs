use super::{Fun, Type};
use crate::{errors::ErrorKind, subst::SubstType, Label, TypeVar};
use std::{collections::HashMap, fmt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant<Ty>
where
    Ty: Type,
{
    variants: HashMap<Label, Ty>,
}

impl<Ty> Type for Variant<Ty>
where
    Ty: Type,
{
    fn into_fun<Ty1>(self) -> Result<Fun<Ty1>, ErrorKind>
    where
        Ty1: Type,
    {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Function Type".to_owned(),
        })
    }
}

impl<Ty> SubstType<Ty> for Variant<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Variant {
            variants: self
                .variants
                .into_iter()
                .map(|(lb, ty1)| (lb, ty1.subst_type(v, ty)))
                .collect(),
        }
        .into()
    }
}

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
