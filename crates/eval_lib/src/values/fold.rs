use super::Value;
use std::{fmt, marker::PhantomData};
use syntax::{
    terms::{Fold as FoldT, Term},
    types::Type,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Fold<V, Ty, T>
where
    V: Value,
    Ty: Type,
    T: Term,
{
    pub ty: Ty,
    pub val: Box<V>,
    phantom: PhantomData<T>,
}

impl<V, Ty, T> Fold<V, Ty, T>
where
    V: Value,
    Ty: Type,
    T: Term,
{
    pub fn new<Ty1, V1>(ty: Ty1, v: V1) -> Fold<Ty, V, T>
    where
        Ty1: Into<Ty>,
        V1: Into<V>,
    {
        Fold {
            ty: ty.into(),
            val: Box::new(v.into()),
            phantom: PhantomData,
        }
    }
}

impl<V, Ty, T> Value for Fold<V, Ty, T>
where
    V: Value,
    Ty: Type,
    T: Term,
    Self: Into<FoldT<T, Ty>>,
{
    type Term = FoldT<T, Ty>;
}

impl<V, Ty, T> From<Fold<V, Ty, T>> for FoldT<T, Ty>
where
    T: Term,
    V: Value,
    Ty: Type,
{
    fn from(fld: Fold<V, Ty, T>) -> FoldT<T, Ty> {
        FoldT::new(*fld.val, fld.ty)
    }
}

impl<V, T, Ty> fmt::Display for Fold<V, T, Ty>
where
    V: Value,
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fold[{}]({})", self.ty, self.val)
    }
}
