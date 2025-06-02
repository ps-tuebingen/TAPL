use super::Type;
use crate::{subst::SubstType, Label, TypeVar};
use common::errors::TypeKind;
use std::{collections::HashMap, fmt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant<Ty>
where
    Ty: Type,
{
    pub variants: HashMap<Label, Ty>,
}

impl<Ty> Variant<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(vars: HashMap<Label, Ty1>) -> Variant<Ty>
    where
        Ty1: Into<Ty>,
    {
        Variant {
            variants: vars.into_iter().map(|(lb, ty)| (lb, ty.into())).collect(),
        }
    }

    pub fn new_single<Ty1>(lb: &str, ty: Ty1) -> Variant<Ty>
    where
        Ty1: Into<Ty>,
    {
        Variant {
            variants: HashMap::from([(lb.to_owned(), ty.into())]),
        }
    }
}

impl<Ty> Type for Variant<Ty>
where
    Ty: Type,
{
    fn knd(&self) -> TypeKind {
        TypeKind::Variant
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
            "<{}>",
            vars.iter()
                .map(|(lb, ty)| format!("{lb}:{ty}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
