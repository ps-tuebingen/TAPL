use super::Value;
use crate::{terms::Left as LeftT, types::Type};
use common::errors::ValueKind;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Left<V, Ty>
where
    V: Value,
    Ty: Type,
{
    pub left_val: Box<V>,
    pub ty: Ty,
}

impl<V, Ty> Left<V, Ty>
where
    V: Value,
    Ty: Type,
{
    pub fn new<V1, Ty1>(val: V1, ty: Ty1) -> Left<V, Ty>
    where
        V1: Into<V>,
        Ty1: Into<Ty>,
    {
        Left {
            left_val: Box::new(val.into()),
            ty: ty.into(),
        }
    }
}

impl<V, Ty> Value for Left<V, Ty>
where
    V: Value,
    Ty: Type,
{
    type Term = LeftT<<V as Value>::Term, Ty>;

    fn knd(&self) -> ValueKind {
        ValueKind::Left
    }
}

impl<V, Ty> From<Left<V, Ty>> for LeftT<<V as Value>::Term, Ty>
where
    V: Value,
    Ty: Type,
{
    fn from(lft: Left<V, Ty>) -> LeftT<<V as Value>::Term, Ty> {
        LeftT::new(*lft.left_val, lft.ty)
    }
}

impl<V, Ty> fmt::Display for Left<V, Ty>
where
    V: Value,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inl({}) as {}", self.left_val, self.ty)
    }
}
