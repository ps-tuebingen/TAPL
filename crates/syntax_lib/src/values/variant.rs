use super::Value;
use crate::{Label, terms::Variant as VariantT, types::Type};
use common::errors::ValueKind;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Variant<V, Ty>
where
    V: Value,
    Ty: Type,
{
    pub label: Label,
    pub val: Box<V>,
    ty: Ty,
}

impl<V, Ty> Variant<V, Ty>
where
    V: Value,
    Ty: Type,
{
    pub fn new<V1, Ty1>(lb: &str, val: V1, ty: Ty1) -> Variant<V, Ty>
    where
        V1: Into<V>,
        Ty1: Into<Ty>,
    {
        Variant {
            label: lb.to_owned(),
            val: Box::new(val.into()),
            ty: ty.into(),
        }
    }
}

impl<V, Ty> Value for Variant<V, Ty>
where
    V: Value,
    Ty: Type,
{
    type Term = VariantT<<V as Value>::Term, Ty>;

    fn knd(&self) -> ValueKind {
        ValueKind::Variant
    }
}

impl<V, Ty> From<Variant<V, Ty>> for VariantT<<V as Value>::Term, Ty>
where
    V: Value,
    Ty: Type,
{
    fn from(var: Variant<V, Ty>) -> VariantT<<V as Value>::Term, Ty> {
        VariantT::new(&var.label, *var.val, var.ty)
    }
}

impl<V, Ty> fmt::Display for Variant<V, Ty>
where
    V: Value,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}={}> as {}", self.label, self.val, self.ty)
    }
}
