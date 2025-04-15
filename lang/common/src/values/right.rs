use super::Value;
use crate::{
    terms::{Right as RightT, Term},
    types::Type,
};
use std::marker::PhantomData;

pub struct Right<V, Ty, T>
where
    V: Value<T>,
    Ty: Type,
    T: Term,
{
    right_val: Box<V>,
    ty: Ty,
    phantom: PhantomData<T>,
}

impl<V, Ty, T> Value<T> for Right<V, Ty, T>
where
    V: Value<T> + Into<T>,
    Ty: Type,
    T: Term,
{
    type Term = RightT<T, Ty>;
}

impl<V, T, Ty> From<Right<V, Ty, T>> for RightT<T, Ty>
where
    T: Term,
    V: Value<T> + Into<T>,
    Ty: Type,
{
    fn from(right: Right<V, Ty, T>) -> RightT<T, Ty> {
        RightT::new(*right.right_val, right.ty)
    }
}
