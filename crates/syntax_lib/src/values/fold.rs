use super::Value;
use crate::errors::ValueKind;
use std::fmt;
use syntax::{terms::Fold as FoldT, types::Type};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Fold<V, Ty>
where
    V: Value,
    Ty: Type,
{
    pub ty: Ty,
    pub val: Box<V>,
}

impl<V, Ty> Fold<V, Ty>
where
    V: Value,
    Ty: Type,
{
    pub fn new<Ty1, V1>(ty: Ty1, v: V1) -> Fold<V, Ty>
    where
        Ty1: Into<Ty>,
        V1: Into<V>,
    {
        Fold {
            ty: ty.into(),
            val: Box::new(v.into()),
        }
    }
}

impl<V, Ty> Value for Fold<V, Ty>
where
    V: Value,
    Ty: Type,
    Self: Into<FoldT<<V as Value>::Term, Ty>>,
{
    type Term = FoldT<<V as Value>::Term, Ty>;

    fn knd(&self) -> ValueKind {
        ValueKind::Fold
    }
}

impl<V, Ty> From<Fold<V, Ty>> for FoldT<<V as Value>::Term, Ty>
where
    V: Value,
    Ty: Type,
{
    fn from(fld: Fold<V, Ty>) -> FoldT<<V as Value>::Term, Ty> {
        FoldT::new(*fld.val, fld.ty)
    }
}

impl<V, Ty> fmt::Display for Fold<V, Ty>
where
    V: Value,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fold[{}]({})", self.ty, self.val)
    }
}
