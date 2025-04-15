use super::Value;
use crate::{
    terms::{Fold as FoldT, Term},
    types::Type,
};
use std::marker::PhantomData;

pub struct Fold<V, Ty, T>
where
    V: Value<T>,
    Ty: Type,
    T: Term,
{
    ty: Ty,
    val: Box<V>,
    phantom: PhantomData<T>,
}

impl<V, Ty, T> Value<T> for Fold<V, Ty, T>
where
    V: Value<T> + Into<T>,
    Ty: Type,
    T: Term,
{
    type Term = FoldT<T, Ty>;
}

impl<V, Ty, T> From<Fold<V, Ty, T>> for FoldT<T, Ty>
where
    T: Term,
    V: Value<T> + Into<T>,
    Ty: Type,
{
    fn from(fld: Fold<V, Ty, T>) -> FoldT<T, Ty> {
        FoldT::new(*fld.val, fld.ty)
    }
}
