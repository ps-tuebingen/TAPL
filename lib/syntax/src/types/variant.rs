use super::Type;
use crate::{
    Label, TypeVar,
    free_vars::FreeTypeVars,
    language::Language,
    span::{Span, Spanned},
    subst::SubstType,
};
use macros::EqNoSpan;
use std::{collections::HashMap, collections::HashSet, fmt};

/// Variant Type
#[derive(Clone, Debug, EqNoSpan)]
pub struct Variant<Lang>
where
    Lang: Language,
{
    /// Labeled variants
    pub variants: HashMap<Label, Lang::Type>,
    /// Source Location
    pub span: Span,
}

impl<Lang> Variant<Lang>
where
    Lang: Language,
{
    /// Create a new variant type with given variants and span
    #[must_use]
    pub fn new<Ty1>(vars: HashMap<Label, Ty1>, span: Span) -> Self
    where
        Ty1: Into<Lang::Type>,
    {
        Self {
            variants: vars.into_iter().map(|(lb, ty)| (lb, ty.into())).collect(),
            span,
        }
    }

    /// Create a new variant type with given single label and type and given span
    pub fn new_single<Ty1>(lb: &str, ty: Ty1, span: Span) -> Self
    where
        Ty1: Into<Lang::Type>,
    {
        Self {
            variants: HashMap::from([(lb.to_owned(), ty.into())]),
            span,
        }
    }
}

impl<Lang> Spanned for Variant<Lang>
where
    Lang: Language,
{
    fn span(&self) -> Span {
        self.span
    }
}

impl<Lang> FreeTypeVars for Variant<Lang>
where
    Lang: Language,
{
    fn free_type_vars(&self, vars: &mut HashSet<TypeVar>) {
        for ty in self.variants.values() {
            ty.free_type_vars(vars);
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
    fn subst_type(mut self, v: &TypeVar, ty: &<Self::Lang as Language>::Type) -> Self::Target {
        self.variants = self
            .variants
            .into_iter()
            .map(|(lb, ty1)| (lb, ty1.subst_type(v, ty)))
            .collect();
        self
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
