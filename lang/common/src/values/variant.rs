use super::{Lambda, Raise, Value};
use crate::{
    errors::ErrorKind,
    terms::{Term, Variant as VariantT},
    types::Type,
    Label,
};
use std::fmt;
use std::marker::PhantomData;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Variant<V, Ty, T>
where
    V: Value<T>,
    Ty: Type,
    T: Term,
{
    label: Label,
    val: Box<V>,
    ty: Ty,
    phantom: PhantomData<T>,
}

impl<V, Ty, T> Value<T> for Variant<V, Ty, T>
where
    V: Value<T> + Into<T>,
    Ty: Type,
    T: Term + From<VariantT<T, Ty>>,
{
    type Term = VariantT<T, Ty>;
    fn into_lambda<Ty1>(self) -> Result<Lambda<T, Ty1>, ErrorKind>
    where
        Ty1: Type,
    {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Lambda Abstraction".to_owned(),
        })
    }

    fn into_raise<Val, Ty1>(self) -> Result<Raise<Val, Ty1, T>, ErrorKind>
    where
        Val: Value<T>,
        Ty1: Type,
    {
        Err(ErrorKind::TypeMismatch {
            found: self.to_string(),
            expected: "Raise".to_owned(),
        })
    }
}

impl<V, Ty, T> From<Variant<V, Ty, T>> for VariantT<T, Ty>
where
    T: Term,
    V: Value<T> + Into<T>,
    Ty: Type,
{
    fn from(var: Variant<V, Ty, T>) -> VariantT<T, Ty> {
        VariantT::new(&var.label, *var.val, var.ty)
    }
}

impl<V, Ty, T> fmt::Display for Variant<V, Ty, T>
where
    V: Value<T>,
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} = {}> as {}", self.label, self.val, self.ty)
    }
}
