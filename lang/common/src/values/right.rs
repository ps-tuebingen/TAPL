use super::Value;
use crate::{
    terms::{Right as RightT, Term},
    types::Type,
};
use std::fmt;
use std::marker::PhantomData;
#[derive(Debug)]
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
    T: Term + From<RightT<T, Ty>>,
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

impl<V, Ty, T> fmt::Display for Right<V, Ty, T>
where
    V: Value<T>,
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inr({}) as {}", self.right_val, self.ty)
    }
}
