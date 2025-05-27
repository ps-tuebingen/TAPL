use super::Value;
use std::{fmt, marker::PhantomData};
use syntax::{
    terms::{Right as RightT, Term},
    types::Type,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Right<V, Ty, T>
where
    V: Value,
    Ty: Type,
    T: Term,
{
    pub right_val: Box<V>,
    ty: Ty,
    phantom: PhantomData<T>,
}

impl<V, Ty, T> Right<V, Ty, T>
where
    V: Value,
    Ty: Type,
    T: Term,
{
    pub fn new<V1, Ty1>(val: V, ty: Ty) -> Right<V, Ty, T>
    where
        V1: Into<V>,
        Ty1: Into<Ty>,
    {
        Right {
            right_val: Box::new(val.into()),
            ty: ty.into(),
            phantom: PhantomData,
        }
    }
}

impl<V, Ty, T> Value for Right<V, Ty, T>
where
    V: Value,
    Ty: Type,
    T: Term,
{
    type Term = RightT<T, Ty>;
}

impl<V, Ty, T> From<Right<V, Ty, T>> for RightT<T, Ty>
where
    V: Value,
    Ty: Type,
    T: Term,
{
    fn from(right: Right<V, Ty, T>) -> RightT<T, Ty> {
        RightT::new(*right.right_val, right.ty)
    }
}

impl<V, Ty, T> fmt::Display for Right<V, Ty, T>
where
    V: Value,
    Ty: Type,
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inr({}) as {}", self.right_val, self.ty)
    }
}
