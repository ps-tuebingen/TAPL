use super::Value;
use crate::{
    Label,
    language::Language,
    span::{Span, Spanned},
    terms::Variant as VariantT,
};
use macros::EqNoSpan;
use std::fmt;

/// Variant value
#[derive(Debug, EqNoSpan, Clone)]
pub struct Variant<Lang>
where
    Lang: Language,
{
    /// Variant label
    pub label: Label,
    /// Inner value
    pub val: Box<Lang::Value>,
    /// annotated type
    ty: Lang::Type,
    /// Source location
    pub span: Span,
}

impl<Lang> Variant<Lang>
where
    Lang: Language,
{
    /// Create a new variant value from a given label, value, type and span
    pub fn new<V, Ty>(lb: &str, val: V, ty: Ty, span: Span) -> Self
    where
        V: Into<Lang::Value>,
        Ty: Into<Lang::Type>,
    {
        Self {
            label: lb.to_owned(),
            val: Box::new(val.into()),
            ty: ty.into(),
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

impl<Lang> Value for Variant<Lang>
where
    Lang: Language,
    VariantT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = VariantT<Lang>;
}

impl<Lang> From<Variant<Lang>> for VariantT<Lang>
where
    Lang: Language,
{
    fn from(var: Variant<Lang>) -> Self {
        Self::new(&var.label, *var.val, var.ty, var.span)
    }
}

impl<Lang> fmt::Display for Variant<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}={}> as {}", self.label, self.val, self.ty)
    }
}
