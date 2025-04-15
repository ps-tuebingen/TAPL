use super::Value;
use crate::{
    terms::{Raise as RaiseT, Term},
    types::Type,
};
use std::marker::PhantomData;

pub struct Raise<V, Ty, T>
where
    V: Value<T>,
    Ty: Type,
    T: Term,
{
    val: Box<V>,
    cont_ty: Ty,
    exception_ty: Ty,
    phantom: PhantomData<T>,
}

impl<V, Ty, T> Value<T> for Raise<V, Ty, T>
where
    V: Value<T> + Into<T>,
    Ty: Type,
    T: Term,
{
    type Term = RaiseT<T, Ty>;
}

impl<V, Ty, T> From<Raise<V, Ty, T>> for RaiseT<T, Ty>
where
    V: Value<T> + Into<T>,
    T: Term,
    Ty: Type,
{
    fn from(r: Raise<V, Ty, T>) -> RaiseT<T, Ty> {
        RaiseT::new(*r.val, r.exception_ty, r.cont_ty)
    }
}
