use super::Value;
use crate::{
    terms::{Left as LeftT, Term},
    types::Type,
};
use std::marker::PhantomData;

pub struct Left<V, Ty, T>
where
    V: Value<T>,
    Ty: Type,
    T: Term,
{
    left_val: V,
    ty: Ty,
    phantom: PhantomData<T>,
}

impl<V, Ty, T> Value<T> for Left<V, Ty, T>
where
    V: Value<T> + Into<T>,
    Ty: Type,
    T: Term,
{
    type Term = LeftT<T, Ty>;
}

impl<V, Ty, T> From<Left<V, Ty, T>> for LeftT<T, Ty>
where
    V: Value<T> + Into<T>,
    T: Term,
    Ty: Type,
{
    fn from(lft: Left<V, Ty, T>) -> LeftT<T, Ty> {
        LeftT::new(lft.left_val, lft.ty)
    }
}
