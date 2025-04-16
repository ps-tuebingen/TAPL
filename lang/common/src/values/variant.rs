use super::Value;
use crate::{language::LanguageTerm, terms::Variant as VariantT, Label};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Variant<T>
where
    T: LanguageTerm,
{
    label: Label,
    val: Box<<T as LanguageTerm>::Value>,
    ty: <T as LanguageTerm>::Type,
}

impl<T> Value for Variant<T>
where
    T: LanguageTerm,
{
    type Term = VariantT<T>;
}

impl<T> From<Variant<T>> for VariantT<T>
where
    T: LanguageTerm,
{
    fn from(var: Variant<T>) -> VariantT<T> {
        VariantT::new(&var.label, *var.val, var.ty)
    }
}

impl<T> fmt::Display for Variant<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} = {}> as {}", self.label, self.val, self.ty)
    }
}
