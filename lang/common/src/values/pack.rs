use super::Value;
use crate::{
    terms::{Pack as PackT, Term},
    types::Type,
};
use std::marker::PhantomData;

pub struct Pack<V, Ty, T>
where
    V: Value<T>,
    Ty: Type,
    T: Term,
{
    inner_ty: Ty,
    val: Box<V>,
    outer_ty: Ty,
    phantom: PhantomData<T>,
}

impl<V, Ty, T> Value<T> for Pack<V, Ty, T>
where
    V: Value<T> + Into<T>,
    Ty: Type,
    T: Term,
{
    type Term = PackT<T, Ty>;
}

impl<V, Ty, T> From<Pack<V, Ty, T>> for PackT<T, Ty>
where
    T: Term,
    V: Value<T> + Into<T>,
    Ty: Type,
{
    fn from(pack: Pack<V, Ty, T>) -> PackT<T, Ty> {
        PackT::new(pack.inner_ty, *pack.val, pack.outer_ty)
    }
}
