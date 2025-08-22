use super::Type;
use crate::{Label, TypeVar, language::Language, subst::SubstType};
use std::{collections::HashMap, fmt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant<Lang>
where
    Lang: Language,
{
    pub variants: HashMap<Label, Lang::Type>,
}

impl<Lang> Variant<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1>(vars: HashMap<Label, Ty1>) -> Variant<Lang>
    where
        Ty1: Into<Lang::Type>,
    {
        Variant {
            variants: vars.into_iter().map(|(lb, ty)| (lb, ty.into())).collect(),
        }
    }

    pub fn new_single<Ty1>(lb: &str, ty: Ty1) -> Variant<Lang>
    where
        Ty1: Into<Lang::Type>,
    {
        Variant {
            variants: HashMap::from([(lb.to_owned(), ty.into())]),
        }
    }
}

impl<Lang> Type for Variant<Lang> where Lang: Language {}

impl<Lang> SubstType for Variant<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &<Self::Lang as Language>::Type) -> Self::Target {
        Variant {
            variants: self
                .variants
                .into_iter()
                .map(|(lb, ty1)| (lb, ty1.subst_type(v, ty)))
                .collect(),
        }
    }
}

impl<Lang> fmt::Display for Variant<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut vars: Vec<(&Label, &Lang::Type)> = self.variants.iter().collect();
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
