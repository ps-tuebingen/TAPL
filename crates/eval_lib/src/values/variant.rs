use super::Value;
use std::{fmt, marker::PhantomData};
use syntax::{
    terms::{Term, Variant as VariantT},
    types::Type,
    Label,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Variant<V, Ty, T>
where
    V: Value,
    Ty: Type,
    T: Term,
{
    pub label: Label,
    pub val: Box<V>,
    ty: Ty,
    phantom: PhantomData<T>,
}

impl<V, Ty, T> Variant<V, Ty, T>
where
    V: Value,
    Ty: Type,
    T: Term,
{
    pub fn new<V1, Ty1>(lb: &str, val: V, ty: Ty) -> Variant<V, Ty, T>
    where
        V1: Into<V>,
        Ty1: Into<Ty>,
    {
        Variant {
            label: lb.to_owned(),
            val: Box::new(val.into()),
            ty: ty.into(),
            phantom: PhantomData,
        }
    }
}

impl<V, Ty, T> Value for Variant<V, Ty, T>
where
    V: Value,
    Ty: Type,
    T: Term + From<V>,
{
    type Term = VariantT<T, Ty>;
}

impl<V, Ty, T> From<Variant<V, Ty, T>> for VariantT<T, Ty>
where
    V: Value,
    Ty: Type,
    T: Term + From<V>,
{
    fn from(var: Variant<V, Ty, T>) -> VariantT<T, Ty> {
        VariantT::new(&var.label, *var.val, var.ty)
    }
}

impl<V, Ty, T> fmt::Display for Variant<V, Ty, T>
where
    V: Value,
    Ty: Type,
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}={}> as {}", self.label, self.val, self.ty)
    }
}
