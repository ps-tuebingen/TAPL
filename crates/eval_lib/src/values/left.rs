use super::Value;
use std::{fmt, marker::PhantomData};
use syntax::{
    terms::{Left as LeftT, Term},
    types::Type,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Left<V, Ty, T>
where
    T: Term,
    V: Value,
    Ty: Type,
{
    pub left_val: Box<V>,
    pub ty: Ty,
    phantom: PhantomData<T>,
}

impl<V, Ty, T> Left<V, Ty, T>
where
    T: Term,
    V: Value,
    Ty: Type,
{
    pub fn new<V1, Ty1>(val: V, ty: Ty) -> Left<V, Ty, T>
    where
        V1: Into<V>,
        Ty1: Into<Ty>,
    {
        Left {
            left_val: Box::new(val.into()),
            ty: ty.into(),
            phantom: PhantomData,
        }
    }
}

impl<V, Ty, T> Value for Left<V, Ty, T>
where
    V: Value,
    Ty: Type,
    T: Term,
{
    type Term = LeftT<T, Ty>;
}

impl<V, Ty, T> From<Left<V, Ty, T>> for LeftT<T, Ty>
where
    T: Term,
    V: Value,
    Ty: Type,
{
    fn from(lft: Left<V, Ty, T>) -> LeftT<T, Ty> {
        LeftT::new(*lft.left_val, lft.ty)
    }
}

impl<V, Ty, T> fmt::Display for Left<V, Ty, T>
where
    T: Term,
    V: Value,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inl({}) as {}", self.left_val, self.ty)
    }
}
