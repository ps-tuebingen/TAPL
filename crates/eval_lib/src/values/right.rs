use super::Value;
use crate::errors::ValueKind;
use std::fmt;
use syntax::{terms::Right as RightT, types::Type};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Right<V, Ty>
where
    V: Value,
    Ty: Type,
{
    pub right_val: Box<V>,
    ty: Ty,
}

impl<V, Ty> Right<V, Ty>
where
    V: Value,
    Ty: Type,
{
    pub fn new<V1, Ty1>(val: V1, ty: Ty1) -> Right<V, Ty>
    where
        V1: Into<V>,
        Ty1: Into<Ty>,
    {
        Right {
            right_val: Box::new(val.into()),
            ty: ty.into(),
        }
    }
}

impl<V, Ty> Value for Right<V, Ty>
where
    V: Value,
    Ty: Type,
{
    type Term = RightT<<V as Value>::Term, Ty>;

    fn knd(&self) -> ValueKind {
        ValueKind::Right
    }
}

impl<V, Ty> From<Right<V, Ty>> for RightT<<V as Value>::Term, Ty>
where
    V: Value,
    Ty: Type,
{
    fn from(right: Right<V, Ty>) -> RightT<<V as Value>::Term, Ty> {
        RightT::new(*right.right_val, right.ty)
    }
}

impl<V, Ty> fmt::Display for Right<V, Ty>
where
    V: Value,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "inr({}) as {}", self.right_val, self.ty)
    }
}
