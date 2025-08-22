use super::Value;
use crate::{Label, language::Language, terms::Variant as VariantT};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Variant<Lang>
where
    Lang: Language,
{
    pub label: Label,
    pub val: Box<Lang::Value>,
    ty: Lang::Type,
}

impl<Lang> Variant<Lang>
where
    Lang: Language,
{
    pub fn new<V, Ty>(lb: &str, val: V, ty: Ty) -> Variant<Lang>
    where
        V: Into<Lang::Value>,
        Ty: Into<Lang::Type>,
    {
        Variant {
            label: lb.to_owned(),
            val: Box::new(val.into()),
            ty: ty.into(),
        }
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
    fn from(var: Variant<Lang>) -> VariantT<Lang> {
        VariantT::new(&var.label, *var.val, var.ty)
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
